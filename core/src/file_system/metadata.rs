use std::fs;
#[cfg(unix)]
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;
use std::{
    fs::{DirEntry, File},
    io::Read,
    path::Path,
};

use std::os::unix::fs::PermissionsExt;

use super::directories::Entry;

use super::utils::{bytes_to_size_string, system_time_string};

/// Metadata de un archivo o directorio, calculada una vez y reutilizada
/// para listar, ordenar y mostrar información.
#[derive(Debug, Clone)]
pub struct Metadata {
    path: PathBuf,
    pub name: String,
    pub ext: Option<String>,
    pub size: u64,
    pub permissions: String,
    pub date_modified: SystemTime,
    pub date_created: SystemTime,
    pub date_accessed: SystemTime,
}

impl Metadata {
    pub fn is_executable(self) -> io::Result<bool> {
        if !fs::metadata(&self.path)?.is_file() {
            return Ok(false);
        }

        if has_exec_permission(&self.path)? {
            return Ok(true);
        }

        let mut header = [0u8; 8];
        let n = File::open(&self.path)?.read(&mut header)?;
        Ok(has_executable_magic(&header[..n]))
    }
    pub fn get_mime_type(self) -> Option<String> {
        get_mime_type(&self.path)
    }
}

/// Extrae la `Metadata` de una entrada del directorio (`DirEntry`), leyendo
/// nombre, extensión, tamaño, permisos y fechas del filesystem.
pub fn get_metadata(entry: &DirEntry) -> Metadata {
    let metadata = entry.metadata().unwrap();
    Metadata {
        path: entry.path(),
        name: entry
            .file_name()
            .into_string()
            .expect("Error converting path to string"),
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

// pub fn get_mime_type(path: &Path) -> Option<String> {
//     let mut buffer = [0u8; 64 * 1024];

//     if let Ok(mut file) = File::open(path)
//         && let Ok(bytes_read) = file.read(&mut buffer)
//     {
//         let mime = infer::get(&buffer[..bytes_read]).map(|kind| kind.mime_type().to_string());

//         return mime;
//     };

//     return None;
// }

pub fn get_mime_type(path: &Path) -> Option<String> {
    let meta = fs::metadata(path).ok()?;
    if meta.is_dir() {
        return Some("inode/directory".to_string());
    }
    if !meta.is_file() {
        return None; // FIFO, sockets, dispositivos: no abrir
    }

    // Leer solo lo necesario (infer rara vez necesita más de ~36 KB)
    let mut buffer = Vec::with_capacity(8192);
    File::open(path)
        .ok()?
        .take(8192)
        .read_to_end(&mut buffer)
        .ok()?;

    // 1) Magic bytes
    if let Some(kind) = infer::get(&buffer) {
        return Some(kind.mime_type().to_string());
    }

    // 2) Extensión
    if let Some(m) = mime_guess::from_path(path).first() {
        return Some(m.essence_str().to_string());
    }

    // 3) ¿Es texto? (sin NULs y UTF-8 válido, tolerando un corte a mitad de carácter)
    if !buffer.contains(&0) {
        match std::str::from_utf8(&buffer) {
            Ok(_) => return Some("text/plain".to_string()),
            Err(e) if e.error_len().is_none() => return Some("text/plain".to_string()),
            _ => {}
        }
    }

    Some("application/octet-stream".to_string())
}

/// Magic bytes de formatos ejecutables conocidos.
fn has_executable_magic(header: &[u8]) -> bool {
    header.starts_with(b"MZ")                          // Windows PE (.exe, .dll)
        || header.starts_with(b"\x7FELF")              // Linux/BSD ELF
        || header.starts_with(&[0xFE, 0xED, 0xFA, 0xCE]) // Mach-O 32-bit
        || header.starts_with(&[0xFE, 0xED, 0xFA, 0xCF]) // Mach-O 64-bit
        || header.starts_with(&[0xCE, 0xFA, 0xED, 0xFE]) // Mach-O 32-bit (LE)
        || header.starts_with(&[0xCF, 0xFA, 0xED, 0xFE]) // Mach-O 64-bit (LE)
        || header.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]) // Mach-O universal / Java class
        || header.starts_with(b"#!") // script con shebang
}

#[cfg(unix)]
fn has_exec_permission(path: &Path) -> io::Result<bool> {
    use std::os::unix::fs::PermissionsExt;
    let meta = fs::metadata(path)?;
    Ok(meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}

#[cfg(windows)]
fn has_exec_permission(path: &Path) -> io::Result<bool> {
    // Windows no tiene bit de ejecución: se decide por extensión (PATHEXT).
    let meta = fs::metadata(path)?;
    if !meta.is_file() {
        return Ok(false);
    }
    let is_exec = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            matches!(
                e.to_ascii_lowercase().as_str(),
                "exe" | "com" | "bat" | "cmd" | "msi" | "ps1" | "vbs" | "scr"
            )
        })
        .unwrap_or(false);
    Ok(is_exec)
}
