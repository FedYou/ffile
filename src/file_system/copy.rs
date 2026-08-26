use std::cmp;
use std::fs::{self, File, read_link, symlink_metadata};
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::process::{ProcessEvent, ProcessId};

struct CopyEntry {
    src: PathBuf,
    destination: PathBuf,
    is_symlink: bool,
    size: u64,
    is_dir: bool,
    mode: u32,
}

enum CopyFileExitStatus {
    Done,
    Cancelled,
}

fn unique_filename(path: &Path, is_dir: bool) -> PathBuf {
    if symlink_metadata(&path).is_err() {
        return path.to_path_buf();
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let stem = path.file_stem().unwrap().to_string_lossy();
    let basename = path.file_name().unwrap().to_string_lossy();

    let file_extension = path.extension().map(|e| e.to_string_lossy().into_owned());

    let mut counter = 1;

    loop {
        let filename = if is_dir {
            format!("{basename} ({counter})")
        } else {
            match &file_extension {
                Some(ext) => format!("{stem} ({counter}).{ext}"),
                None => format!("{stem} ({counter})"),
            }
        };

        let candidate = parent.join(filename);

        if symlink_metadata(&candidate).is_err() {
            return candidate;
        }

        counter += 1;
    }
}

fn get_all_entries_on_directory(
    src: PathBuf,
    destination: &Path,
) -> Result<Vec<CopyEntry>, std::io::Error> {
    let mut entries: Vec<CopyEntry> = vec![];

    for entry in fs::read_dir(&src)? {
        let Ok(entry) = entry else { continue };
        let file_type = entry.file_type()?;

        let Some(destination_ok) = entry
            .path()
            .strip_prefix(&src)
            .ok()
            .map(|rel| destination.join(rel))
        else {
            continue;
        };

        let src_path = if file_type.is_symlink() {
            read_link(entry.path())?
        } else {
            entry.path()
        };

        let metadata = if file_type.is_symlink() {
            symlink_metadata(entry.path())?
        } else {
            entry.metadata()?
        };

        entries.push(CopyEntry {
            src: src_path,
            destination: destination_ok.clone(),
            is_symlink: file_type.is_symlink(),
            is_dir: file_type.is_dir(),
            size: metadata.len(),
            mode: metadata.permissions().mode(),
        });

        if file_type.is_dir() {
            entries.append(&mut get_all_entries_on_directory(
                entry.path(),
                &destination_ok,
            )?);
        }
    }

    Ok(entries)
}

fn get_all_entries_on_sources(
    sources: Vec<PathBuf>,
    destination: &Path,
) -> Result<Vec<CopyEntry>, std::io::Error> {
    let mut entries: Vec<CopyEntry> = vec![];

    for source in sources {
        let metadata = symlink_metadata(&source)?;

        let Some(name) = source.file_name() else {
            continue;
        };

        let dir_destination = unique_filename(destination.join(name).as_path(), metadata.is_dir());

        entries.push(CopyEntry {
            src: source.clone(),
            destination: dir_destination.clone(),
            is_symlink: metadata.file_type().is_symlink(),
            is_dir: metadata.file_type().is_dir(),
            size: metadata.len(),
            mode: metadata.permissions().mode(),
        });

        if metadata.file_type().is_dir() {
            let mut source_entries = get_all_entries_on_directory(source, &dir_destination)?;
            entries.append(&mut source_entries);
        }
    }

    Ok(entries)
}

#[cfg(target_os = "linux")]
unsafe fn copy_file_range_wrapper(
    source_fd: std::os::unix::io::RawFd,
    destination_fd: std::os::unix::io::RawFd,
    len: usize,
) -> io::Result<usize> {
    let ret = unsafe {
        libc::copy_file_range(
            source_fd,
            std::ptr::null_mut(),
            destination_fd,
            std::ptr::null_mut(),
            len,
            0,
        )
    };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret as usize)
    }
}

const COPY_CHUNK_MIN: usize = 4 * 1024 * 1024;
const COPY_TARGET_UPDATES: u64 = 128;

async fn copy_file(
    entry: &CopyEntry,
    entries_done: &mut usize,
    bytes_total: u64,
    bytes_done: &mut u64,
    process_id: ProcessId,
    tx: &tokio::sync::mpsc::Sender<ProcessEvent>,
    cancel_flag: &AtomicBool,
    started_at: Instant,
) -> Result<CopyFileExitStatus, io::Error> {
    let mut input = File::open(&entry.src)?;
    let mut output = File::create(&entry.destination)?;

    let result: io::Result<CopyFileExitStatus> = async {
        let mut bytes_copied: u64 = 0;
        let entry_size = entry.size;

        #[cfg(target_os = "linux")]
        {
            use std::os::unix::io::AsRawFd;
            let source_fd = input.as_raw_fd();
            let destination_fd = output.as_raw_fd();

            let chunk_size = cmp::max(COPY_CHUNK_MIN, (entry_size / COPY_TARGET_UPDATES) as usize);

            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    return Ok(CopyFileExitStatus::Cancelled);
                }

                let remaining = (entry_size - bytes_copied) as usize;
                if remaining == 0 {
                    break;
                }

                let to_copy = cmp::min(chunk_size, remaining);

                match unsafe { copy_file_range_wrapper(source_fd, destination_fd, to_copy) } {
                    Ok(0) => break,
                    Ok(bytes) => {
                        bytes_copied += bytes as u64;
                        *bytes_done += bytes as u64;

                        let progress = *bytes_done as f64 / bytes_total as f64 * 100.0;

                        let elapsed = started_at.elapsed().as_secs_f64();
                        let speed = if elapsed > 0.0 {
                            // Se divide los bytes ya escritos por el tiempo transcurrido, por ej:
                            // bytes_done=10  elepsed=5s;
                            // 10b
                            // --- =  2b/s
                            // 4/s
                            *bytes_done as f64 / elapsed
                        } else {
                            0.0
                        };
                        // Es la cantidad que falta de bytes
                        let remaining_bytes = bytes_total.saturating_sub(*bytes_done);
                        let eta = if speed > 0.0 {
                            // Se divide por velocidad para dar un estimado, por ej:
                            // remaining=10b speed=2b/s;
                            // 10b    10
                            // ---  = --- =  5s
                            // 2b/s   2/s
                            Some(Duration::from_secs_f64(remaining_bytes as f64 / speed))
                        } else {
                            None
                        };

                        let _ = tx
                            .send(ProcessEvent::Progress {
                                id: process_id,
                                progress: Some(progress),
                                bytes_done: Some(*bytes_done),
                                bytes_total: Some(bytes_total),
                                speed: Some(speed),
                                eta,
                                message: Some(format!(
                                    "Copying {}",
                                    entry.src.file_name().unwrap().display()
                                )),
                                entries_total: None,
                                entries_done: Some(*entries_done),
                            })
                            .await;
                    }
                    Err(_) => break,
                }
            }
        }

        if bytes_copied < entry_size {
            let chunk_size = cmp::max(COPY_CHUNK_MIN, (entry_size / COPY_TARGET_UPDATES) as usize);
            let mut buffer = vec![0u8; chunk_size];

            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    return Ok(CopyFileExitStatus::Cancelled);
                }

                let bytes_read = input.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }

                output.write_all(&buffer[..bytes_read])?;

                *bytes_done += bytes_read as u64;

                let progress = *bytes_done as f64 / bytes_total as f64 * 100.0;

                let elapsed = started_at.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    *bytes_done as f64 / elapsed
                } else {
                    0.0
                };

                let remaining_bytes = bytes_total.saturating_sub(*bytes_done);
                let eta = if speed > 0.0 {
                    Some(Duration::from_secs_f64(remaining_bytes as f64 / speed))
                } else {
                    None
                };

                let _ = tx
                    .send(ProcessEvent::Progress {
                        id: process_id,
                        progress: Some(progress),
                        bytes_done: Some(*bytes_done),
                        bytes_total: Some(bytes_total),
                        speed: Some(speed),
                        eta,
                        message: Some(format!(
                            "Copying {}",
                            entry.src.file_name().unwrap().display()
                        )),
                        entries_total: None,
                        entries_done: Some(*entries_done),
                    })
                    .await;
            }
        }

        fs::set_permissions(&entry.destination, fs::Permissions::from_mode(entry.mode))?;

        Ok(CopyFileExitStatus::Done)
    }
    .await;

    match result {
        Err(_) | Ok(CopyFileExitStatus::Cancelled) => {
            let _ = fs::remove_file(&entry.destination);
        }
        _ => {}
    }

    result
}

pub async fn copy(
    sources: Vec<PathBuf>,
    destination: PathBuf,
    tx: tokio::sync::mpsc::Sender<ProcessEvent>,
    cancel_flag: &AtomicBool,
    process_id: ProcessId,
) -> Result<(), io::Error> {
    if sources.is_empty() {
        return Err(io::Error::other("No elements to copy"));
    }

    let _ = tx
        .send(ProcessEvent::Progress {
            id: process_id,
            progress: None,
            bytes_done: None,
            bytes_total: None,
            speed: None,
            eta: None,
            message: Some("Obtained list of files and directories...".to_string()),
            entries_total: None,
            entries_done: None,
        })
        .await;

    let entries = get_all_entries_on_sources(sources, &destination)?;
    let mut entries_done: usize = 0;

    let bytes_total: u64 = entries.iter().map(|e| e.size).sum();
    let mut bytes_done: u64 = 0;
    let started_at = Instant::now();

    let mut errors: Vec<String> = vec![];
    let mut cancelled = false;

    let _ = tx
        .send(ProcessEvent::Progress {
            id: process_id,
            progress: None,
            bytes_done: None,
            bytes_total: None,
            speed: None,
            eta: None,
            message: None,
            entries_total: Some(entries.len()),
            entries_done: None,
        })
        .await;

    for entry in entries {
        if cancel_flag.load(Ordering::Relaxed) {
            cancelled = true;
            break;
        }

        if entry.is_symlink {
            match std::os::unix::fs::symlink(&entry.src, &entry.destination) {
                Ok(()) => {
                    bytes_done += entry.size;
                    entries_done += 1;
                }
                Err(err) => errors.push(format!("{}: {err}", entry.destination.display())),
            }
        } else if entry.is_dir {
            match fs::create_dir_all(&entry.destination) {
                Ok(()) => {
                    bytes_done += entry.size;
                    entries_done += 1;
                }
                Err(err) => errors.push(format!("{}: {err}", entry.destination.display())),
            }
        } else {
            match copy_file(
                &entry,
                &mut entries_done,
                bytes_total,
                &mut bytes_done,
                process_id,
                &tx,
                cancel_flag,
                started_at,
            )
            .await
            {
                Ok(CopyFileExitStatus::Done) => {
                    entries_done += 1;
                }
                Ok(CopyFileExitStatus::Cancelled) => {
                    cancelled = true;
                    break;
                }
                Err(err) => errors.push(format!("{}: {err}", entry.destination.display())),
            }
        }
    }

    if cancelled {
        return Ok(());
    }

    if !errors.is_empty() {
        return Err(io::Error::other(errors.join("\n")));
    }

    let _ = tx
        .send(ProcessEvent::Progress {
            id: process_id,
            progress: None,
            bytes_done: None,
            bytes_total: Some(bytes_total),
            speed: None,
            eta: None,
            message: None,
            entries_total: None,
            entries_done: Some(entries_done),
        })
        .await;

    Ok(())
}
