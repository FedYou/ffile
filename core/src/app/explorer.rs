use std::{
    fs::{File, create_dir_all},
    io::Error,
    path::PathBuf,
};

use crate::{
    file_system::{
        directories::{
            Entry, Sort, get_bookmarks, get_dir_entries, get_parent_dir, get_pwd,
            resolve_existing_dir,
        },
        metadata::get_serialize_metadata,
    },
    get_config,
};

pub struct ExplorerOptions {
    pub show_hidden: bool,
    pub sort: Sort,
    pub invert_sort: bool,
    pub search: Option<String>,
}

/// Estado del explorador: directorio actual, entradas del directorio,
/// bookmarks y la metadata del elemento seleccionado.
pub struct ExplorerState {
    /// Directorio de trabajo actual que se está explorando.
    pub pwd: PathBuf,
    /// Bookmarks del sidebar
    pub bookmarks: Vec<(String, Option<PathBuf>)>,
    /// (archivos/directorios) del directorio actual.
    pub entries: Vec<Entry>,
    ///
    pub visible_entries: Vec<Entry>,
    /// Metadata serializada del elemento seleccionado, lista para mostrar.
    pub metadata: Option<Vec<(String, String)>>,
    pub options: ExplorerOptions,
}

impl Default for ExplorerState {
    /// Inicializa el explorador leyendo el pwd actual, listando sus
    /// entradas según la config (orden, orden invertido, ocultos) y
    /// calculando la metadata de la primera entrada si existe alguna.
    fn default() -> Self {
        let config = get_config();
        let pwd: PathBuf = get_pwd();

        let mut state = Self {
            pwd,
            bookmarks: get_bookmarks(),
            entries: Vec::new(),
            visible_entries: Vec::new(),
            metadata: None,
            options: ExplorerOptions {
                show_hidden: config.show_hidden_files.clone(),
                sort: config.sort.clone(),
                invert_sort: config.invert_sort,
                search: None,
            },
        };
        state.update_entries();
        state.update_metadata(0);

        state
    }
}

impl ExplorerState {
    /// Devuelve el directorio actual.
    pub fn get_pwd(&self) -> PathBuf {
        self.pwd.clone()
    }

    /// Resuelve la ruta actual y si no existe retrocede hasta encontrar una válida
    pub fn update_pwd(&mut self, path: Option<PathBuf>) {
        self.pwd = resolve_existing_dir(if let Some(_path) = path {
            _path
        } else {
            self.get_pwd()
        });

        // Actualiza la ruta del proceso de trabajo actual de la app
        let _ = std::env::set_current_dir(self.get_pwd());
    }

    /// Reemplaza la lista de entidades del directorio actual.
    pub fn update_entries(&mut self) {
        self.entries = get_dir_entries(self.get_pwd())
            .expect(format!("Cannot get contents of {:?} directory", self.get_pwd()).as_str());
        self.apply_filters_entries();
    }

    pub fn update_metadata(&mut self, index: usize) {
        self.metadata = if !self.visible_entries.is_empty()
            && let Some(entry) = &self.visible_entries.get(index)
        {
            Some(get_serialize_metadata(entry))
        } else {
            None
        };
    }

    /// Crea un archivo o directorio basado en `full_path`. Si `input_value` termina
    /// en "/", es directorio sino un archivo
    pub fn create_entry(&self, full_path: PathBuf, input_value: &String) -> Result<(), Error> {
        if input_value.ends_with("/") {
            create_dir_all(full_path)?;
            return Ok(());
        }

        let parent = get_parent_dir(full_path.clone());
        create_dir_all(parent)?;
        File::create(full_path)?;

        Ok(())
    }

    pub fn apply_filters_entries(&mut self) {
        let mut entries: Vec<Entry> = self.entries.clone();

        match self.options.sort {
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
        if self.options.invert_sort {
            entries.reverse();
        }

        // → Al final, los directorios van siempre antes que los archivos.
        entries.sort_by_key(|e| {
            if e.is_file {
                return 1;
            }
            0
        });

        if !self.options.show_hidden {
            entries.retain(|e| !e.metadata.name.starts_with("."));
        }

        if let Some(filter_str) = &self.options.search {
            entries.retain(|e| e.metadata.name.contains(filter_str));
        }

        self.visible_entries = entries
    }
}
