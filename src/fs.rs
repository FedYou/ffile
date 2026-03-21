use crate::utils::permissions_mode_to_string;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub enum ItemType {
    Dir,
    File,
}

#[derive(Debug)]
pub struct ItemPath<'a> {
    path: Option<PathBuf>,
    name: &'a str,
    icon: &'a str,
    _type: ItemType,
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum EntryType {
    Dir,
    File,
}

#[derive(Debug, Clone)]
pub struct EntryMetadata {
    size: u64,
    permissions: String,
    date_modified: SystemTime,
    date_created: SystemTime,
    date_accessed: SystemTime,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub is_symlink: bool,
    pub _type: EntryType,
    pub metadata: EntryMetadata,
}

#[derive(Debug)]
pub struct DirContentResult {
    pub dirs: Vec<Entry>,
    pub files: Vec<Entry>,
}

pub enum Sort {
    Date,
    Size,
    Name,
}

pub struct DirContent<'a> {
    pub path: &'a str,
    pub sort: Sort,
    pub invert: bool,
    pub show_hidden: bool,
}

pub fn get_dir_content(dir_content: DirContent) -> Result<Vec<Entry>, std::io::Error> {
    let mut entries: Vec<Entry> = fs::read_dir(dir_content.path)?
        .filter_map(|f| f.ok())
        .map(|f| {
            let metadata = f.metadata().unwrap();
            let file_type = f.file_type().unwrap();
            let mut _type: EntryType = EntryType::File;

            if file_type.is_dir() {
                _type = EntryType::Dir
            }

            return Entry {
                path: f.path(),
                name: f.file_name().into_string().expect("REASON"),
                is_symlink: file_type.is_symlink(),
                _type,
                metadata: EntryMetadata {
                    size: metadata.len(),
                    permissions: permissions_mode_to_string(metadata.permissions().mode()),
                    date_modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                    date_created: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
                    date_accessed: metadata.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                },
            };
        })
        .collect();

    match dir_content.sort {
        Sort::Name => {
            entries.sort_by_key(|f| f.name.clone());
        }
        Sort::Size => {
            entries.sort_by_key(|f| f.metadata.size.clone());
        }
        Sort::Date => {
            entries.sort_by_key(|f| f.metadata.date_modified);
        }
    }
    if dir_content.invert {
        entries.reverse();
    }

    entries.sort_by_key(|f| match f._type {
        EntryType::Dir => 0,
        EntryType::File => 1,
    });

    if !dir_content.show_hidden {
        entries.retain(|f| !f.name.starts_with("."));
    }

    Ok(entries)
}

pub fn get_user_dirs() -> Vec<ItemPath<'static>> {
    let mut dirs: Vec<ItemPath> = Vec::new();

    if let Some(user_dirs) = directories::UserDirs::new() {
        let home = ItemPath {
            path: Some(user_dirs.home_dir().to_path_buf()),
            name: "Home",
            icon: " ",
            _type: ItemType::Dir,
        };
        let desktop = ItemPath {
            path: user_dirs.desktop_dir().map(|p| p.to_path_buf()),
            name: "Desktop",
            icon: "󰇅 ",
            _type: ItemType::Dir,
        };

        let picture = ItemPath {
            path: user_dirs.picture_dir().map(|p| p.to_path_buf()),
            name: "Pictures",
            icon: " ",
            _type: ItemType::Dir,
        };

        let document = ItemPath {
            path: user_dirs.document_dir().map(|p| p.to_path_buf()),
            name: "Documents",
            icon: "󱔗 ",
            _type: ItemType::Dir,
        };

        let audio = ItemPath {
            path: user_dirs.audio_dir().map(|p| p.to_path_buf()),
            name: "Music",
            icon: " ",
            _type: ItemType::Dir,
        };

        let video = ItemPath {
            path: user_dirs.video_dir().map(|p| p.to_path_buf()),
            name: "Videos",
            icon: "󰕧 ",
            _type: ItemType::Dir,
        };

        let download = ItemPath {
            path: user_dirs.download_dir().map(|p| p.to_path_buf()),
            name: "Downloads",
            icon: "󰇚 ",
            _type: ItemType::Dir,
        };

        dirs.push(home);
        dirs.push(desktop);
        dirs.push(download);
        dirs.push(picture);
        dirs.push(audio);
        dirs.push(video);
        dirs.push(document);
    }

    return dirs;
}
