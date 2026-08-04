use std::fs::DirEntry;
use std::time::SystemTime;

use std::os::unix::fs::PermissionsExt;

use super::directories::Entry;

use super::utils::{bytes_to_size_string, system_time_string};

/// Metadata de un archivo o directorio, calculada una vez y reutilizada
/// para listar, ordenar y mostrar información.
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

/// Convierte el modo de permisos (bits) a su representación estilo `ls`,
/// por ejemplo `rwxr-xr--`. Recorre los 9 bits leyendo/escribiendo/ejecutando
/// para usuario, grupo y otros.
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

/// Devuelve los permisos del archivo como string (ej. `rwxr-xr--`).
fn get_permissions(metadata: &std::fs::Metadata) -> String {
    permissions_mode_to_string(metadata.permissions().mode())
}

/// Extrae la `Metadata` de una entrada del directorio (`DirEntry`), leyendo
/// nombre, extensión, tamaño, permisos y fechas del filesystem.
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

/// Convierte la `Metadata` de un `Entry` en una lista de pares
/// `(etiqueta, valor)` lista para mostrar en el panel Metadata (tipo,
/// nombre, tamaño, permisos, fechas, ruta y extensión).
pub fn get_serialize_metadata(entry: &Entry) -> Vec<(String, String)> {
    let mut list = Vec::new();

    list.push((
        "Type".to_string(),
        format!(
            "{} {}",
            if entry.is_file { "File" } else { "Dir" },
            if entry.is_symlink { "(symlink)" } else { "" }
        ),
    ));

    list.push(("Name".to_string(), entry.metadata.name.clone()));

    list.push((
        "Size".to_string(),
        bytes_to_size_string(entry.metadata.size),
    ));

    list.push((
        "Permissions".to_string(),
        entry.metadata.permissions.clone(),
    ));

    list.push((
        "Date Modified".to_string(),
        system_time_string(entry.metadata.date_modified),
    ));

    list.push((
        "Date Created".to_string(),
        system_time_string(entry.metadata.date_created),
    ));
    list.push((
        "Date Accessed".to_string(),
        system_time_string(entry.metadata.date_accessed),
    ));
    list.push(("Path".to_string(), entry.path.to_str().unwrap().into()));

    if entry.is_file {
        if let Some(ext) = entry.metadata.ext.clone() {
            list.push(("Ext".to_string(), ext));
        }
    }

    list
}
