use std::fs::{File, create_dir_all};
use std::io::Error;
use std::path::PathBuf;
use std::sync::MutexGuard;

use crate::file_system::metadata::get_serialize_metadata;
use crate::index::SelectionIndex;
use crate::input::Input;
use crate::processes::ProcessesManager;
use crate::shortcuts::KeyCode;
use arboard::Clipboard;
use notify::{RecommendedWatcher, Watcher};

use crate::actions::Action;
use crate::config::get_config;
use crate::file_system::directories::{
    Entry, Sort, get_bookmarks, get_dir_entries, get_parent_dir, get_pwd, resolve_existing_dir,
};
use crate::global;
use crate::{file_system, watcher};
use enum_map::{Enum, EnumMap};

// Enums y Structs

/// Lista de los Paneles de la app
#[derive(PartialEq, Enum, Copy, Clone)]
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

/// Estado del panel para crear de archivos/directorios.
pub enum CreateState {
    /// Popup cerrado.
    Closed,
    /// Popup abierto, con el input activo para escribir
    Editing,
    /// Ocurrió un error al crear; guarda el error para mostrarlo.
    Error(Error),
}

impl Default for CreateState {
    fn default() -> Self {
        CreateState::Closed
    }
}

impl CreateState {
    /// Cambia el estado a `Error`
    fn fail(&mut self, err: Error) {
        *self = CreateState::Error(err);
    }
    /// Abre el popup en modo edición.
    fn open(&mut self) {
        *self = CreateState::Editing;
    }
    /// Cierra el popup.
    fn close(&mut self) {
        *self = CreateState::Closed;
    }
}

pub enum DeleteState {
    Closed,
    Open,
    Error(Error),
}

impl Default for DeleteState {
    fn default() -> Self {
        DeleteState::Closed
    }
}

impl DeleteState {
    /// Cambia el estado a `Error`
    fn fail(&mut self, err: Error) {
        *self = DeleteState::Error(err);
    }
    /// Abre el popup en modo edición.
    fn open(&mut self) {
        *self = DeleteState::Open;
    }
    /// Cierra el popup.
    fn close(&mut self) {
        *self = DeleteState::Closed;
    }
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

/// Acciones que deben ejecutarse "afuera" del loop principal (abrir una app
/// externa o un editor). Se guardan acá y el loop principal las consume y
/// las limpia después de consumirlas.
pub struct PendingExternalActions {
    /// Archivo listo para abrirse con la app externa por defecto.
    pub open_with_app: Option<Entry>,
    /// Archivo listo para abrirse en el editor externo configurado.
    pub open_with_editor: Option<PathBuf>,
}

impl Default for PendingExternalActions {
    fn default() -> Self {
        Self {
            open_with_app: None,
            open_with_editor: None,
        }
    }
}

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
    fn get_pwd(&self) -> PathBuf {
        self.pwd.clone()
    }

    /// Resuelve la ruta actual y si no existe retrocede hasta encontrar una válida
    fn update_pwd(&mut self, path: Option<PathBuf>) {
        self.pwd = resolve_existing_dir(if let Some(_path) = path {
            _path
        } else {
            self.get_pwd()
        });

        // Actualiza la ruta del proceso de trabajo actual de la app
        let _ = std::env::set_current_dir(self.get_pwd());
    }

    /// Reemplaza la lista de entidades del directorio actual.
    fn update_entries(&mut self) {
        self.entries = get_dir_entries(self.get_pwd())
            .expect(format!("Cannot get contents of {:?} directory", self.get_pwd()).as_str());
        self.apply_filters_entries();
    }

    fn update_metadata(&mut self, index: usize) {
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
    fn create_entry(&self, full_path: PathBuf, input_value: &String) -> Result<(), Error> {
        if input_value.ends_with("/") {
            create_dir_all(full_path)?;
            return Ok(());
        }

        let parent = get_parent_dir(full_path.clone());
        create_dir_all(parent)?;
        File::create(full_path)?;

        Ok(())
    }

    fn delete_entry(&self, path: PathBuf) -> Result<(), Error> {
        file_system::delete::delete(path)
    }

    fn apply_filters_entries(&mut self) {
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

// App

pub struct App<'a> {
    pub title: &'static str,
    pub quit: bool,
    /// `true` cuando la terminal es más chica que `MIN_WIDTH_APP`/`MIN_HEIGHT_APP`.
    /// Mientras sea `true`, `execute_action` ignora todas las acciones salvo salir.
    pub is_small: bool,
    pub active_panel: Panel,
    /// Índice de selección de cada panel, independiente entre sí.
    pub panels: EnumMap<Panel, SelectionIndex>,
    pub explorer: ExplorerState,
    /// Watcher del filesystem: vigila el directorio actual para detectar cambios.
    watcher: MutexGuard<'a, RecommendedWatcher>,
    /// Portapapeles del sistema (si hay uno disponible).
    clipboard: Option<Clipboard>,
    pub file_clipboard: FileClipboardState,
    pub input: Input,
    pub pending_external_actions: PendingExternalActions,
    pub create_state: CreateState,
    pub delete_state: DeleteState,
    pub processes: ProcessesManager,
}

impl<'a> App<'a> {
    // Inicialización

    /// Crea la App con su estado inicial y empieza a vigilar el explorer.pwd con el watcher.
    pub fn new() -> Self {
        let state = ExplorerState::default();

        let mut watcher = watcher::get_watcher();

        let _ = watcher.watch(&state.pwd, notify::RecursiveMode::NonRecursive);

        let mut panels: EnumMap<Panel, SelectionIndex> = EnumMap::default();

        panels[Panel::SideBar].setup(state.bookmarks.len(), Some(0), None);
        App {
            title: &global::APP_TITLE,
            quit: false,
            is_small: false,
            active_panel: Panel::SideBar,
            panels,
            explorer: state,
            input: Input::default(),
            watcher,
            clipboard: Clipboard::new().ok(),
            file_clipboard: FileClipboardState::default(),
            pending_external_actions: PendingExternalActions::default(),
            create_state: CreateState::default(),
            delete_state: DeleteState::default(),
            processes: ProcessesManager::new(),
        }
    }
    /// Se activa al haber un cambio en el directorio (disparado por el watcher).
    /// Revalida el pwd (por si fue borrado/movido), recarga las entradas,
    /// ajusta el índice seleccionado y recalcula la metadata mostrada.
    pub fn update(&mut self) {
        self.explorer.update_pwd(None);
        self.explorer.update_entries();

        if !self.explorer.visible_entries.is_empty() {
            self.panels[Panel::Files].setup(self.explorer.visible_entries.len(), None, None);
        }

        // Selecciona el último elemento si el índice anterior es mayor a la
        // cantidad de entries actuales (evita quedar apuntando "afuera" de la lista)
        self.panels[Panel::Files].clamp_selected(self.explorer.visible_entries.len());

        self.explorer
            .update_metadata(self.panels[Panel::Files].selected);
    }

    // Entrada de acciones (keymap -> Action)

    /// Punto de entrada único para procesar una `Action` que se dispara por un
    /// atajo de teclado. Si la terminal está muy chica (`is_small`), solo se
    /// permite salir; el resto de las acciones se ignoran.
    pub fn execute_action(&mut self, action: Action) {
        if action == Action::Quit {
            self.quit = true
        }

        if self.is_small {
            return;
        }

        match action {
            Action::Up => self.action_up(),
            Action::Down => self.action_down(),
            Action::Left => self.action_left(),
            Action::Right => self.action_right(),
            Action::Confirm => self.action_confirm(),
            Action::Cancel => self.action_cancel(),
            Action::Focus => self.action_focus(),
            Action::Editor => self.action_open_in_editor(),

            Action::ToggleShowFiles => self.action_toggle_show_files(),
            Action::ToggleSort => self.action_toggle_sort(),
            Action::ToggleInvertSort => self.action_toggle_invert_invert(),
            Action::Metadata => self.action_focus_metadata_panel(),
            Action::CopyMetadata => self.action_copy_selected_metadata(),

            Action::New => self.action_focus_new(),
            Action::Delete => self.action_focus_delete(),

            // Action::MoveInputCursorLeft => self.input.move_cursor_left(),
            // Action::MoveInputC/ Action::MoveInputCursorRight => self.input.move_cursor_right(),
            Action::RemoveInputChar => self.input.remove_char(),

            Action::AddClipboard => self.action_add_clipboard(),
            Action::RemoveClipboard => self.action_remove_clipboard(),
            Action::ClearFileClipboard => self.action_clear_clipboard(),
            Action::FocusClipboard => self.action_focus_file_clipboard(),
            Action::ExitFileClipboard => self.action_exit_clipboard(),

            Action::Processes => self.action_focus_processes(),
            Action::CancelProcess => self.action_cancel_process(),
            Action::Copy => self.action_add_copy_process(),
            _ => {}
        }
    }

    /// Maneja teclas "de texto" cuando el panel
    /// activo espera texto libre, como el panel `Create`.
    pub fn execute_input_key(&mut self, key: KeyCode) {
        match self.active_panel {
            Panel::New => {
                if let Some(ch) = key.as_char() {
                    self.input.set_char(ch);
                }
            }
            _ => {}
        }
    }

    // Getters de datos derivados del estado

    /// Retorna el Entry actualmente seleccionado en el FilePanel, si existe.
    fn get_current_entry(&self) -> Option<&Entry> {
        self.explorer
            .visible_entries
            .get(self.panels[Panel::Files].selected)
    }

    /// Retorna (clave, valor) de metadata actual seleccionada dentro del
    /// panel Metadata. Devuelve `None` si el panel no está activo o es invalido.
    fn get_current_metadata_selected(&self) -> Option<(String, String)> {
        if !self.panels[Panel::Metadata].valid {
            return None;
        }

        self.explorer
            .metadata
            .as_ref()?
            .get(self.panels[Panel::Metadata].selected)
            .cloned()
    }

    /// Obtiene Bookmark actualmente seleccionado en el Sidebar, si el panel es válido.
    fn get_current_bookmarks(&self) -> Option<(String, Option<PathBuf>)> {
        if !self.panels[Panel::SideBar].valid {
            return None;
        }
        self.explorer
            .bookmarks
            .get(self.panels[Panel::SideBar].selected)
            .cloned()
    }

    // Funciones de control internas

    /// Cambia el pwd a `path`: reubica el watcher, recarga entries y metadata.
    /// Usada por navegación (abrir carpeta, ir al padre, abrir bookmark).
    fn open_directory(&mut self, path: PathBuf) {
        let _ = self.watcher.unwatch(&self.explorer.pwd);

        self.explorer.update_pwd(Some(path));

        let _ = self
            .watcher
            .watch(&self.explorer.pwd, notify::RecursiveMode::NonRecursive);

        self.explorer.update_entries();

        if !self.explorer.visible_entries.is_empty() {
            self.panels[Panel::Files].setup(self.explorer.visible_entries.len(), Some(0), None);
        }

        self.explorer
            .update_metadata(self.panels[Panel::Files].selected);
    }

    /// Copia `value` al portapapeles del sistema, si hay uno disponible.
    fn copy_clipboard(&mut self, value: String) {
        if let Some(clipboard) = self.clipboard.as_mut() {
            let _ = clipboard.set_text(value);
        }
    }

    // Acciones: movimiento

    /// Mueve la selección hacia arriba dentro del panel activo.
    /// Si el panel activo es el FilePanel actualiza la metadata seleccionada.
    fn action_up(&mut self) {
        self.panels[self.active_panel].prev();
        match self.active_panel {
            Panel::Files => {
                self.explorer
                    .update_metadata(self.panels[Panel::Files].selected);
            }
            Panel::Processes => {
                self.panels[Panel::ProcessInfo].selected = 0;
            }
            _ => {}
        }
    }

    /// Mueve la selección hacia abajo dentro del panel activo.
    /// Si el panel activo es el FilePanel actualiza la metadata seleccionada.
    fn action_down(&mut self) {
        self.panels[self.active_panel].next();
        match self.active_panel {
            Panel::Files => {
                self.explorer
                    .update_metadata(self.panels[Panel::Files].selected);
            }
            Panel::Processes => {
                self.panels[Panel::ProcessInfo].selected = 0;
            }
            _ => {}
        }
    }

    fn action_left(&mut self) {
        match self.active_panel {
            // Va al directorio padre del pwd actual
            Panel::Files => {
                let path = get_parent_dir(self.explorer.pwd.clone());
                self.open_directory(path);
            }
            // Mover el cursor a la izquierda
            Panel::New => self.input.move_cursor_left(),
            Panel::ProcessInfo => self.action_exit_process_info(),
            _ => {}
        }
    }

    fn action_right(&mut self) {
        match self.active_panel {
            Panel::SideBar => self.action_open_selected_bookmark(),
            // Abrir una carpeta
            Panel::Files => {
                self.action_open_selected_entry();
            }
            // Mover el cursor a la derecha
            Panel::New => self.input.move_cursor_right(),
            Panel::Processes => self.action_open_process_info(),

            _ => {}
        }
    }

    fn action_focus(&mut self) {
        match self.active_panel {
            Panel::SideBar => self.action_focus_files(),
            Panel::Files => self.action_focus_sidebar(),
            Panel::Processes => self.action_open_process_info(),
            Panel::ProcessInfo => self.action_exit_process_info(),
            _ => {}
        }
    }

    // Acciones: navegación y apertura de directorios/archivos

    fn action_focus_files(&mut self) {
        self.active_panel = Panel::Files;

        if self.explorer.visible_entries.is_empty() {
            self.panels[Panel::Files].reset();
        } else {
            self.panels[Panel::Files].setup(self.explorer.visible_entries.len(), None, None);
        }
    }

    fn action_focus_sidebar(&mut self) {
        self.active_panel = Panel::SideBar
    }

    fn action_confirm(&mut self) {
        match self.active_panel {
            Panel::SideBar => self.action_open_selected_bookmark(),
            Panel::Files => self.action_open_selected_entry(),
            Panel::New => self.action_confirm_create(),
            Panel::Delete => self.action_confirm_delete(),
            Panel::Processes => self.action_open_process_info(),
            _ => {}
        }
    }

    fn action_cancel(&mut self) {
        match self.active_panel {
            Panel::New => self.action_cancel_create(),
            Panel::Delete => self.action_cancel_delete(),
            Panel::Metadata => self.action_exit_metadata(),
            Panel::Clipboard => self.action_exit_clipboard(),
            Panel::Processes => self.action_exit_process(),
            Panel::ProcessInfo => self.action_exit_process_info(),
            _ => {}
        }
    }

    /// Navega al path del bookmark seleccionado en el Sidebar y le pasa el
    /// focus al FilePanel.
    pub fn action_open_selected_bookmark(&mut self) {
        if let Some((_, path)) = self.get_current_bookmarks() {
            if let Some(p) = path {
                self.open_directory(p.clone());
                self.active_panel = Panel::Files;
            }
        }
    }

    /// Si el entry seleccionado es un archivo, lo marca para abrirse con
    /// una app externa (`pending_external_actions`). Si es un directorio,
    /// navega dentro de él.
    fn action_open_selected_entry(&mut self) {
        let Some(entry) = self.get_current_entry() else {
            return;
        };

        let path = entry.path.clone();

        if entry.is_file {
            self.pending_external_actions.open_with_app = Some(entry.clone());
            return;
        }

        self.open_directory(path);
    }

    /// Si el entry seleccionado es un archivo, lo marca para abrirse en el
    /// editor externo configurado.
    fn action_open_in_editor(&mut self) {
        self.pending_external_actions.open_with_editor = Some(self.explorer.get_pwd());
    }

    fn action_toggle_show_files(&mut self) {
        self.explorer.options.show_hidden = !self.explorer.options.show_hidden;
        self.explorer.apply_filters_entries();
        self.explorer
            .update_metadata(self.panels[Panel::Files].selected);

        if !self.explorer.visible_entries.is_empty() {
            self.panels[Panel::Files].setup(self.explorer.visible_entries.len(), None, None);
        }
    }

    fn action_toggle_sort(&mut self) {
        self.explorer.options.sort = match self.explorer.options.sort {
            Sort::Date => Sort::Size,
            Sort::Size => Sort::Name,
            Sort::Name => Sort::Date,
        };

        self.explorer.apply_filters_entries();
        self.explorer
            .update_metadata(self.panels[Panel::Files].selected);
    }

    fn action_toggle_invert_invert(&mut self) {
        self.explorer.options.invert_sort = !self.explorer.options.invert_sort;
        self.explorer.apply_filters_entries();
        self.explorer
            .update_metadata(self.panels[Panel::Files].selected);
    }

    // Acciones: panel Metadata

    /// Le da el focus al panel Metadata y selecciona el primer ítem.
    fn action_focus_metadata_panel(&mut self) {
        if self.active_panel == Panel::Metadata {
            return;
        }

        self.active_panel = Panel::Metadata;
        if let Some(metadata) = self.explorer.metadata.as_ref() {
            self.panels[Panel::Metadata].setup(metadata.len(), None, None);
        } else {
            self.panels[Panel::Metadata].reset();
        }
    }

    /// Copia al portapapeles el valor de la metadata actual seleccionada.
    fn action_copy_selected_metadata(&mut self) {
        if self.active_panel != Panel::Metadata {
            return;
        }

        if let Some((_, value)) = self.get_current_metadata_selected() {
            self.copy_clipboard(value);
        }
    }

    /// Desactiva el panel de metadata y activa el File Panel
    fn action_exit_metadata(&mut self) {
        self.active_panel = Panel::Files;
        self.panels[Panel::Metadata].reset();
    }

    // Acciones: panel Create

    /// Abre el panel de creación de archivo/directorio, limpiando el input
    /// y mostrando el mensaje inicial del popup.
    fn action_focus_new(&mut self) {
        self.active_panel = Panel::New;
        self.input.clear();
        self.create_state.open();
    }

    /// Confirma la creación del archivo/directorio ingresado en el input.
    /// Si el input está vacío, simplemente cierra el panel. Si hay un error
    /// al crear, lo muestra en el popup y mantiene el panel abierto.
    fn action_confirm_create(&mut self) {
        if let CreateState::Error(_) = self.create_state {
            return;
        }

        let input_value = self.input.to_string();

        if input_value.trim().is_empty() {
            self.active_panel = Panel::Files;
            self.input.clear();
            return;
        }

        let full_path = self.explorer.pwd.join(input_value.clone());

        if let Err(err) = self.explorer.create_entry(full_path, &input_value) {
            self.create_state.fail(err);
        } else {
            self.active_panel = Panel::Files;
            self.create_state.close();
        }

        self.input.clear();
    }

    /// Cancela la creación: cierra el popup, vuelve al FilePanel y
    /// descarta el texto ingresado en el input.
    fn action_cancel_create(&mut self) {
        self.create_state.close();
        self.active_panel = Panel::Files;
        self.input.clear();
    }

    // Acciones: panel Delete
    fn action_focus_delete(&mut self) {
        if self.explorer.visible_entries.is_empty() {
            return;
        }

        self.active_panel = Panel::Delete;
        self.delete_state.open();
    }

    fn action_confirm_delete(&mut self) {
        if let DeleteState::Error(_) = self.delete_state {
            return;
        }

        if let Some(entry) = self.get_current_entry() {
            if let Err(err) = self.explorer.delete_entry(entry.path.clone()) {
                self.delete_state.fail(err);
            } else {
                self.active_panel = Panel::Files;
                self.delete_state.close();
                self.update();
            }
        }
    }

    fn action_cancel_delete(&mut self) {
        self.delete_state.close();
        self.active_panel = Panel::Files;
    }

    // Acciones: FileClipboard

    /// Añade el entry seleccionado del file_panel a la lista del file_clipboard.
    fn action_add_clipboard(&mut self) {
        if let Some(entry) = self.get_current_entry() {
            let entry_path = entry.path.clone();

            if self
                .file_clipboard
                .list
                .iter()
                .find(|i| &i.path == &entry_path)
                .is_some()
            {
                return;
            };

            self.file_clipboard.add(entry_path);
            self.panels[Panel::Clipboard].count = self.file_clipboard.len();
            self.panels[Panel::Clipboard].set_selected(0);
        }
    }

    /// Elimina el entry seleccionado de la lista del FileClipboard
    fn action_remove_clipboard(&mut self) {
        if self.active_panel == Panel::Files
            && let Some(entry) = self.get_current_entry()
            && self.file_clipboard.remove_exist(&entry.path.clone())
        {
            self.panels[Panel::Clipboard].count = self.file_clipboard.len();
            self.panels[Panel::Clipboard].clamp_selected(self.file_clipboard.len());
        }

        if self.active_panel != Panel::Clipboard {
            return;
        }

        self.file_clipboard
            .remove(self.panels[Panel::Clipboard].selected);

        self.panels[Panel::Clipboard].count = self.file_clipboard.len();
        self.panels[Panel::Clipboard].clamp_selected(self.file_clipboard.len());

        if self.file_clipboard.len() == 0 {
            self.action_exit_clipboard();
        }
    }

    /// Limpia toda la lista del FileClipboard
    fn action_clear_clipboard(&mut self) {
        if self.active_panel != Panel::Clipboard {
            return;
        }

        self.file_clipboard.clear();

        self.panels[Panel::Clipboard].clamp_selected(self.file_clipboard.len());

        self.action_exit_clipboard();
    }

    /// Le da focus al FileClipboard
    fn action_focus_file_clipboard(&mut self) {
        if self.file_clipboard.len() > 0 {
            self.active_panel = Panel::Clipboard;
            self.panels[Panel::Clipboard].valid = true;
        }
    }

    /// Sale del FileClipboard y le da focus al FilePanel
    fn action_exit_clipboard(&mut self) {
        self.active_panel = Panel::Files;
        self.panels[Panel::Clipboard].valid = false;
    }

    // Process panel

    fn action_focus_processes(&mut self) {
        if self.active_panel == Panel::Processes {
            return;
        }

        self.active_panel = Panel::Processes;

        if self.processes.len() > 0 {
            self.panels[Panel::Processes].setup(self.processes.len(), None, None);
            self.panels[Panel::ProcessInfo].valid = true;
        }
    }

    fn action_exit_process(&mut self) {
        self.active_panel = Panel::Files;
        self.panels[Panel::Processes].valid = false;
        self.panels[Panel::ProcessInfo].valid = false;
    }

    fn action_cancel_process(&mut self) {
        if let Some((_, process)) = self
            .processes
            .get_index(self.panels[Panel::Processes].selected)
        {
            let id = process.id;
            self.processes.cancel(&id);
        }
    }

    fn action_exit_process_info(&mut self) {
        self.active_panel = Panel::Processes;
        self.panels[Panel::ProcessInfo].selected = 0;
    }

    fn action_open_process_info(&mut self) {
        self.active_panel = Panel::ProcessInfo;
    }

    fn action_add_copy_process(&mut self) {
        if self.file_clipboard.len() == 0 {
            return;
        }

        self.processes
            .spawn_copy(self.file_clipboard.get(), self.explorer.get_pwd());
    }
}
