use std::fs::DirEntry;
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::fs::dir::Entry;
use crate::fs::utils::{bytes_to_size_string, system_time_string};

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub ext: Option<String>,
    pub size: u64,
    pub permissions: String,
    pub date_modified: SystemTime,
    pub date_created: SystemTime,
    pub date_accessed: SystemTime,
}

#[derive(PartialEq, Clone)]
pub struct SerializeMetadata {
    pub name: String,
    pub value: String,
}

fn permissions_mode_to_string(mode: u32) -> String {
    let mut s = String::new();
    let flags = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    for (bit, ch) in flags {
        s.push(if mode & bit != 0 { ch } else { '-' });
    }

    s
}

fn get_permissions(metadata: &std::fs::Metadata) -> String {
    #[cfg(unix)]
    {
        permissions_mode_to_string(metadata.permissions().mode())
    }

    #[cfg(windows)]
    {
        if metadata.permissions().readonly() {
            "r--r--r--".to_string()
        } else {
            "rwxrwxrwx".to_string()
        }
    }
}

pub fn get_metadata(entry: &DirEntry) -> Metadata {
    let metadata = entry.metadata().unwrap();

    Metadata {
        name: entry.file_name().into_string().expect("REASON"),
        ext: entry
            .path()
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_string()),
        size: metadata.len(),
        permissions: get_permissions(&metadata),
        date_modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        date_created: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
        date_accessed: metadata.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
    }
}

pub fn serialize_metadata(entry: &Entry) -> Vec<SerializeMetadata> {
    let mut list: Vec<SerializeMetadata> = vec![];

    list.push(SerializeMetadata {
        name: "Type".to_string(),
        value: format!(
            "{} {}",
            if entry.is_file { "File" } else { "Dir" },
            if entry.is_symlink { "(symlink)" } else { "" }
        ),
    });

    list.push(SerializeMetadata {
        name: "Name".to_string(),
        value: entry.metadata.name.clone(),
    });

    list.push(SerializeMetadata {
        name: "Size".to_string(),
        value: bytes_to_size_string(entry.metadata.size),
    });

    list.push(SerializeMetadata {
        name: "Permissions".to_string(),
        value: entry.metadata.permissions.clone(),
    });

    list.push(SerializeMetadata {
        name: "Date Modified".to_string(),
        value: system_time_string(entry.metadata.date_modified),
    });

    list.push(SerializeMetadata {
        name: "Date Created".to_string(),
        value: system_time_string(entry.metadata.date_created),
    });

    list.push(SerializeMetadata {
        name: "Date Accessed".to_string(),
        value: system_time_string(entry.metadata.date_accessed),
    });

    list.push(SerializeMetadata {
        name: "Path".to_string(),
        value: entry.path.to_str().unwrap().into(),
    });

    list
}
