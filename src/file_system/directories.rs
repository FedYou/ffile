use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use super::metadata::{Metadata, get_metadata};

/// Elemento del sidebar: un bookmark con su nombre visible y su ruta.
/// La ruta puede ser `None` si el directorio no existe en el sistema.
#[derive(Clone)]
pub struct UserItem {
    pub path: Option<PathBuf>,
    pub name: String,
}

/// Criterio de orden de las entradas de un directorio.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Date,
    Size,
    Name,
}

/// Opciones para listar un directorio: qué directorio leer, cómo ordenar
/// y filtrar, y si se muestran archivos ocultos.
pub struct EntriesOptions {
    pub path: PathBuf,
    pub sort: Sort,
    pub invert: bool,
    pub show_hidden: bool,
    pub filter: Option<String>,
}

/// Una entrada dentro de un directorio (archivo o subdirectorio), con su
/// ruta, si es symlink/archivo y su metadata.
#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub is_symlink: bool,
    pub is_file: bool,
    pub metadata: Metadata,
}

/// Lee el directorio indicado en `options` y devuelve sus entradas
/// ordenadas y filtradas (según orden, invertido, ocultos y filtro).
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

    // → Al final, los directorios van siempre antes que los archivos.
    entries.sort_by_key(|e| {
        if e.is_file {
            // → Archivo: va después de los directorios
            return 1;
        }
        // → Directorio: va primero
        0
    });

    if !options.show_hidden {
        // → Filtra los archivos ocultos (empiezan con ".").
        entries.retain(|e| !e.metadata.name.starts_with("."));
    }

    if let Some(filter_str) = &options.filter {
        // → Filtra por nombre si se indicó un filtro de búsqueda.
        entries.retain(|e| e.metadata.name.contains(filter_str));
    }

    Ok(entries)
}

/// Devuelve el directorio home del usuario actual.
fn get_user_home_dir() -> PathBuf {
    directories::UserDirs::new()
        .expect("User's HOME folder not found")
        .home_dir()
        .to_path_buf()
}

/// Devuelve el directorio de trabajo actual; si falla, el home del usuario.
pub fn get_pwd() -> PathBuf {
    match std::env::current_dir() {
        Ok(p) => p,
        Err(_) => get_user_home_dir(),
    }
}

/// Construye la lista de bookmarks del sidebar a partir de los directorios
/// del usuario (Home, Desktop, Downloads, etc.), incluyendo solo los que
/// existen en el sistema.
pub fn get_bookmarks() -> Vec<(String, Option<PathBuf>)> {
    let mut bookmarks: Vec<(String, Option<PathBuf>)> = Vec::new();

    if let Some(user_dirs) = directories::UserDirs::new() {
        bookmarks.push(("Home".to_string(), Some(user_dirs.home_dir().to_path_buf())));

        if let Some(p) = user_dirs.desktop_dir() {
            bookmarks.push(("Desktop".to_string(), Some(p.to_path_buf())));
        }

        if let Some(p) = user_dirs.download_dir() {
            bookmarks.push(("Downloads".to_string(), Some(p.to_path_buf())));
        }

        if let Some(p) = user_dirs.picture_dir() {
            bookmarks.push(("Pictures".to_string(), Some(p.to_path_buf())));
        }

        if let Some(p) = user_dirs.audio_dir() {
            bookmarks.push(("Music".to_string(), Some(p.to_path_buf())));
        }

        if let Some(p) = user_dirs.video_dir() {
            bookmarks.push(("Videos".to_string(), Some(p.to_path_buf())));
        }

        if let Some(p) = user_dirs.document_dir() {
            bookmarks.push(("Documents".to_string(), Some(p.to_path_buf())));
        }
    }

    bookmarks
}

/// Devuelve el directorio de configuración de la app (depende del SO).
pub fn get_config_dir() -> PathBuf {
    let proj =
        directories::ProjectDirs::from("com", "fedyou", "ffile").expect("ProjectDirs failed");

    proj.config_dir().to_path_buf()
}

/// Devuelve el directorio padre de `path`. Si la ruta no tiene padre
/// (ej. la raíz), devuelve la misma ruta.
pub fn get_parent_dir(path: PathBuf) -> PathBuf {
    path.parent()
        // Si tiene directorio padre, lo retorna
        .map(|p| p.to_path_buf())
        // De lo contrario, devuelve la misma ruta ingresada
        .unwrap_or(path)
}

/// Comprueba que `path` sea un directorio válido. Si no existe (o es un
/// archivo), sube recursivamente hasta encontrar un directorio existente.
/// Se usa para que la app nunca quede apuntando a una ruta inexistente.
pub fn resolve_existing_dir(path: PathBuf) -> PathBuf {
    // Valida si la ruta actual existe y es un directorio
    let mut is_invalid = false;

    match fs::metadata(&path) {
        Ok(entry) => {
            if entry.is_file() {
                is_invalid = true;
            }
        }
        Err(_) => is_invalid = true,
    }

    if is_invalid {
        return path
            .parent()
            // → Sube un nivel y vuelve a intentar, hasta dar con una ruta válida
            .map(|p| resolve_existing_dir(p.to_path_buf()))
            .unwrap_or(path);
    }

    path
}
