use std::path::PathBuf;

use enum_map::Enum;

// Enums y Structs

/// Lista de los Paneles de la app
#[derive(PartialEq, Enum, Copy, Clone, Debug)]
pub enum Panel {
    /// Sidebar con los bookmarks (Home, Desktop, ...).
    SideBar,
    /// Lista de archivos y directorios del directorio actual.
    Files,
    /// Metadata del elemento seleccionado.
    Metadata,
    /// Popup para crear archivos/directorios.
    New,
    /// Popup para eliminar archivos/directorios.
    Delete,
    /// Clipboard de los archivos/directorios.
    Clipboard,
    /// Panel de los procesos de archivos
    Processes,
    ProcessInfo,
}

pub struct FileClipboardItem {
    pub path: PathBuf,
    pub exist: bool,
}

pub struct FileClipboardState {
    pub list: Vec<FileClipboardItem>,
    pub in_progress: bool,
}

impl Default for FileClipboardState {
    fn default() -> Self {
        FileClipboardState {
            list: vec![],
            in_progress: false,
        }
    }
}

impl FileClipboardState {
    pub fn add(&mut self, path: PathBuf) {
        let exist = path.as_path().exists();

        self.list.insert(0, FileClipboardItem { path, exist });
    }

    /// Actualiza el estado de si esa ruta de archivo/directorio existe
    pub fn update_states(&mut self) {
        self.list = self
            .list
            .iter()
            .map(|i| FileClipboardItem {
                path: i.path.clone(),
                exist: i.path.as_path().exists(),
            })
            .collect()
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
        self.update_states();
    }

    pub fn remove_exist(&mut self, path: &PathBuf) -> bool {
        if let Some(index) = self.list.iter().position(|item| &item.path == path) {
            self.list.remove(index);
            self.update_states();
            return true;
        } else {
            return false;
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn get(&self) -> Vec<PathBuf> {
        self.list.iter().map(|i| i.path.clone()).collect()
    }

    pub fn exist(&self, path: &PathBuf) -> bool {
        self.list.iter().find(|i| &i.path == path).is_some()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}
