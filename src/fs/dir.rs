use std::{fs, path::PathBuf};

use super::metadata::{get_metadata, Metadata};

pub struct UserItem {
    path: Option<PathBuf>,
    name: String,
}

pub enum Sort {
    Date,
    Size,
    Name,
}

pub struct EntriesOptions {
    pub path: String,
    pub sort: Sort,
    pub invert: bool,
    pub show_hidden: bool,
    pub filter: Option<String>,
}

pub struct Entry {
    path: PathBuf,
    is_symlink: bool,
    is_file: bool,
    metadata: Metadata,
}

pub fn get_dir_entries(options: EntriesOptions) -> Result<Vec<Entry>, std::io::Error> {
    let mut entries: Vec<Entry> = fs::read_dir(options.path)?
        .filter_map(|e| e.ok())
        .map(|e| {
            let file_type = e.file_type().unwrap();

            return Entry {
                path: e.path(),
                is_symlink: file_type.is_symlink(),
                is_file: file_type.is_file(),
                metadata: get_metadata(&e),
            };
        })
        .collect();

    match options.sort {
        Sort::Name => {
            entries.sort_by_key(|e| e.metadata.name.clone());
        }
        Sort::Size => {
            entries.sort_by_key(|e| e.metadata.size.clone());
        }
        Sort::Date => {
            entries.sort_by_key(|e| e.metadata.date_modified.clone());
        }
    }
    if options.invert {
        entries.reverse();
    }

    entries.sort_by_key(|e| {
        if e.is_file {
            //file
            return 1;
        }
        //dir
        0
    });

    if !options.show_hidden {
        //hidden
        entries.retain(|e| !e.metadata.name.starts_with("."));
    }

    if let Some(filter_str) = &options.filter {
        entries.retain(|e| e.metadata.name.contains(filter_str));
    }

    Ok(entries)
}

pub fn get_user_home_dir() -> PathBuf {
    directories::UserDirs::new()
        .expect("User's HOME folder not found")
        .home_dir()
        .to_path_buf()
}

pub fn get_user_dirs() -> Vec<UserItem> {
    let mut dirs: Vec<UserItem> = Vec::new();

    if let Some(user_dirs) = directories::UserDirs::new() {
        dirs.push(UserItem {
            path: Some(user_dirs.home_dir().to_path_buf()),
            name: "Home".to_string(),
        });

        dirs.push(UserItem {
            path: user_dirs.desktop_dir().map(|p| p.to_path_buf()),
            name: "Desktop".to_string(),
        });
        dirs.push(UserItem {
            path: user_dirs.download_dir().map(|p| p.to_path_buf()),
            name: "Downloads".to_string(),
        });
        dirs.push(UserItem {
            path: user_dirs.picture_dir().map(|p| p.to_path_buf()),
            name: "Pictures".to_string(),
        });
        dirs.push(UserItem {
            path: user_dirs.audio_dir().map(|p| p.to_path_buf()),
            name: "Music".to_string(),
        });
        dirs.push(UserItem {
            path: user_dirs.video_dir().map(|p| p.to_path_buf()),
            name: "Videos".to_string(),
        });
        dirs.push(UserItem {
            path: user_dirs.document_dir().map(|p| p.to_path_buf()),
            name: "Documents".to_string(),
        });
    }

    dirs
}
