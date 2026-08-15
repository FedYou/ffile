use std::fs::{File, create_dir_all};
use std::io::Error;
use std::path::PathBuf;

use crate::file_system::metadata::get_serialize_metadata;
use crate::index::SelectionIndex;
use crate::input::Input;
use arboard::Clipboard;
use crossterm::event::KeyCode;
use notify::{RecommendedWatcher, Watcher};

use crate::config::get_config;
use crate::file_system::directories::{
    EntriesOptions, Entry, get_bookmarks, get_dir_entries, get_parent_dir, get_pwd,
    resolve_existing_dir,
};
use crate::global::{self, MIN_HEIGHT_APP, MIN_WIDTH_APP};
use crate::keymap::Action;
use crate::tui;
use enum_map::{Enum, EnumMap};

// ─── Enums y Structs  ────────────────────────────────────────

/// Lista de los Paneles de la app
#[derive(PartialEq, Enum, Copy, Clone)]
pub enum Panel {
    /// Sidebar con los bookmarks (Home, Desktop, ...).
    SideBar,
    /// Lista de archivos y directorios del directorio actual.
    FilePanel,
    /// Metadata del elemento seleccionado.
    Metadata,
    /// Popup para crear archivos/directorios.
    Create,
    // Clipboard de los archivos/directorios
    FileClipboard,
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

impl Default for CreateState {
    fn default() -> Self {
        CreateState::Closed
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

        self.list.push(FileClipboardItem { path, exist });
    }

    // Actualiza el estado de si esa ruta de archivo/directorio existe
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

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn get(&self) -> Vec<PathBuf> {
        self.list.iter().map(|i| i.path.clone()).collect()
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
    pub open_with_app: Option<PathBuf>,
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

/// Estado del explorador: directorio actual, entradas del directorio,
/// bookmarks y la metadata del elemento seleccionado.
pub struct ExplorerState {
    /// Directorio de trabajo actual que se está explorando.
    pub pwd: PathBuf,
    /// Bookmarks del sidebar
    pub bookmarks: Vec<(String, Option<PathBuf>)>,
    // pinned: HashMap<String, Option<PathBuf>>,
    /// Entidades (archivos/directorios) del directorio actual.
    pub entries: Vec<Entry>,
    /// Metadata serializada del elemento seleccionado, lista para mostrar.
    pub metadata: Option<Vec<(String, String)>>,
}

impl ExplorerState {
    /// Devuelve el directorio actual.
    fn get_pwd(&self) -> PathBuf {
        self.pwd.clone()
    }

    /// Cambia el directorio actual.
    fn set_pwd(&mut self, path: PathBuf) {
        self.pwd = path
    }

    /// Reemplaza la lista de entidades del directorio actual.
    fn set_entries(&mut self, entries: Vec<Entry>) {
        self.entries = entries
    }
}

impl Default for ExplorerState {
    /// Inicializa el explorador leyendo el pwd actual, listando sus
    /// entradas según la config (orden, orden invertido, ocultos) y
    /// calculando la metadata de la primera entrada si existe alguna.
    fn default() -> Self {
        let config = get_config().lock().unwrap();
        let pwd: PathBuf = get_pwd();

        let dir_entries_options = EntriesOptions {
            path: pwd.clone(),
            sort: config.sort.clone(),
            invert: config.invert_sort.clone(),
            show_hidden: config.show_hidden_files.clone(),
            filter: None,
        };

        let entries: Vec<Entry> = get_dir_entries(dir_entries_options)
            .expect(format!("Cannot get contents of {:?} directory", pwd.clone()).as_str());

        let file_panel_index: Option<usize> = if !entries.is_empty() { Some(0) } else { None };

        let metadata = if let Some(index) = file_panel_index {
            if entries.is_empty() {
                None
            } else {
                Some(get_serialize_metadata(&entries.get(index).unwrap()))
            }
        } else {
            None
        };

        Self {
            pwd,
            bookmarks: get_bookmarks(),
            entries,
            metadata,
        }
    }
}

// ─── ★ Estado principal de la app ──────────────────────────────────────────

pub struct App {
    pub title: &'static str,
    pub exit: bool,
    /// `true` cuando la terminal es más chica que `MIN_WIDTH_APP`/`MIN_HEIGHT_APP`.
    /// Mientras sea `true`, `execute_action` ignora todas las acciones salvo salir.
    pub is_small: bool,
    pub active_panel: Panel,
    /// Índice de selección de cada panel, independiente entre sí.
    pub panels: EnumMap<Panel, SelectionIndex>,
    pub explorer: ExplorerState,
    /// Watcher del filesystem: vigila el directorio actual para detectar cambios.
    watcher: RecommendedWatcher,
    /// Portapapeles del sistema (si hay uno disponible).
    clipboard: Option<Clipboard>,
    pub file_clipboard: FileClipboardState,
    pub input: Input,
    pub pending_external_actions: PendingExternalActions,
    pub create_state: CreateState,
}

impl App {
    // ── Inicialización  ─────────────────────────────────────────────────

    /// Crea la App con su estado inicial y empieza a vigilar el explorer.pwd con el watcher.
    pub fn new(mut watcher: RecommendedWatcher) -> Self {
        let state = ExplorerState::default();
        let _ = watcher.watch(&state.pwd, notify::RecursiveMode::NonRecursive);

        let mut panels: EnumMap<Panel, SelectionIndex> = EnumMap::default();

        panels[Panel::SideBar].setup(state.bookmarks.len(), Some(0), None);
        App {
            title: &global::TITLE,
            exit: false,
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
        }
    }

    // ── Ciclo (render / update) ──────────────────────────────

    /// Se llama en cada frame del loop principal.
    /// Y inicializa el renderizado del tui.
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        let term_area = frame.area();

        if term_area.width < MIN_WIDTH_APP || term_area.height < MIN_HEIGHT_APP {
            self.is_small = true;
        } else {
            self.is_small = false
        }

        tui::render(frame, self);
    }

    ///  Se activa al haber un cambio en el directorio (disparado por el watcher).
    /// Revalida el pwd (por si fue borrado/movido), recarga las entradas,
    /// ajusta el índice seleccionado y recalcula la metadata mostrada.
    pub fn update(&mut self) {
        // Resuelve la ruta actual y si no existe retrocede hasta encontrar una válida
        let pwd = self.explorer.get_pwd();
        self.explorer.set_pwd(resolve_existing_dir(pwd));

        // Actualiza la ruta del proceso de trabajo actual de la app
        let _ = std::env::set_current_dir(self.explorer.get_pwd());

        // Obtiene las opciones para leer las entries del directorio
        let options = self.get_entries_option();

        // Mensaje de error temporal si no se puede obtener el contenido del directorio
        let expect_msg = format!(
            "Cannot get contents of {} directory",
            self.explorer.get_pwd().to_str().unwrap()
        );

        // Actualiza la lista de entries
        self.explorer
            .set_entries(get_dir_entries(options).expect(&expect_msg));

        if !self.explorer.entries.is_empty() {
            self.panels[Panel::FilePanel].setup(self.explorer.entries.len(), None, None);
        }

        // Selecciona el último elemento si el índice anterior es mayor a la
        // cantidad de entries actuales (evita quedar apuntando "afuera" de la lista)
        self.panels[Panel::FilePanel].clamp_seleted(self.explorer.entries.len());

        self.update_metadata_current();
    }

    // ── 🛠 Funciones de control internas ─────────────────────────────────

    /// ▸ Cambia el pwd a `path`: reubica el watcher, recarga entries y metadata.
    /// Usada por navegación (abrir carpeta, ir al padre, abrir bookmark).
    fn open_directory(&mut self, path: PathBuf) {
        let valid_dir = resolve_existing_dir(path);

        let _ = self.watcher.unwatch(&self.explorer.pwd);
        let _ = self
            .watcher
            .watch(&valid_dir, notify::RecursiveMode::NonRecursive);

        self.explorer.pwd = valid_dir;

        // Actualiza la ruta del proceso de trabajo actual de la app
        let _ = std::env::set_current_dir(self.explorer.get_pwd());

        let options = self.get_entries_option();

        let expect_msg = format!(
            "Cannot get contents of {} directory",
            self.explorer.pwd.to_str().unwrap()
        );

        self.explorer.entries = get_dir_entries(options).expect(&expect_msg);

        if !self.explorer.entries.is_empty() {
            self.panels[Panel::FilePanel].setup(self.explorer.entries.len(), Some(0), None);
        }
        self.update_metadata_current();
    }

    /// ▸ Recalcula `explorer.metadata` en base a la entry actualmente seleccionada.
    fn update_metadata_current(&mut self) {
        self.explorer.metadata = self.get_current_metadata()
    }

    /// ▸ Copia `value` al portapapeles del sistema, si hay uno disponible.
    fn copy_clipboard(&mut self, value: String) {
        if let Some(clipboard) = self.clipboard.as_mut() {
            let _ = clipboard.set_text(value);
        }
    }

    // ──  Getters de datos derivados del estado ─────────────────────────

    // Entries (FilePanel)

    /// Retorna el Entry actualmente seleccionado en el FilePanel, si existe.
    fn get_current_entry(&self) -> Option<&Entry> {
        self.explorer
            .entries
            .get(self.panels[Panel::FilePanel].selected)
    }

    /// Retorna las opciones de lectura de directorio (orden, ocultos, etc.) a
    /// partir de la config global y el pwd actual.
    fn get_entries_option(&self) -> EntriesOptions {
        let config = get_config().lock().unwrap();
        EntriesOptions {
            path: self.explorer.pwd.clone(),
            sort: config.sort.clone(),
            invert: config.invert_sort.clone(),
            show_hidden: config.show_hidden_files,
            filter: None,
        }
    }

    // Metadata

    /// Retorna la Metadata serializada del entry actual seleccionado en el FilePanel.
    fn get_current_metadata(&self) -> Option<Vec<(String, String)>> {
        self.get_current_entry().map(get_serialize_metadata)
    }

    /// Retorna (clave, valor) de metadata actualseleccionada dentro del
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

    // Sidebar

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

    // ── Sistema de archivos ────────────────────────────────────────────

    /// Crea un archivo o directorio basado en `full_path`. Si `input_value` termina
    /// en "/", se interpreta como directorio (crea toda la ruta con
    /// `create_dir_all`); en caso contrario crea un archivo, asegurando antes
    /// que exista el directorio padre.
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

    // ── Entrada de acciones (keymap -> Action) ─────────────────────────

    /// Punto de entrada único para procesar una `Action` que se dispara por un
    /// atajo de teclado. Si la terminal está muy chica (`is_small`), solo se
    /// permite salir; el resto de las acciones se ignoran.
    pub fn execute_action(&mut self, action: Action) {
        if action == Action::Exit {
            self.exit = true
        }

        if self.is_small {
            return;
        }

        match action {
            // Movimiento Arriba y Abajo
            Action::MoveUp => self.action_move_up(),
            Action::MoveDown => self.action_move_down(),

            // Abrir directorios y elementos
            Action::OpenBookmark => self.action_open_selected_bookmark(),
            Action::OpenEntry => self.action_open_selected_entry(),
            Action::OpenEditor => self.action_open_selected_in_editor(),
            Action::GoToParentDir => self.action_go_to_parent_dir(),

            // Cambio entre el panel del Sidebar y FilePanel
            Action::FocusFilePanel => self.action_focus_file_panel(),
            Action::FocusSideBar => self.action_focus_sidebar_panel(),

            // Panel Metadata
            Action::FocusMetadata => self.action_focus_metadata_panel(),
            Action::CopyMetadata => self.action_copy_selected_metadata(),
            Action::ExitMetadata => self.action_exit_metadata(),
            // Panel Create
            Action::OpenCreatePanel => self.action_open_create_panel(),
            Action::ConfirmCreate => self.action_confirm_create(),
            Action::CancelCreate => self.action_cancel_create(),
            // Movimiento del cursor del input
            Action::MoveInputCursorLeft => self.input.move_cursor_left(),
            Action::MoveInputCursorRight => self.input.move_cursor_right(),
            Action::RemoveInputChar => self.input.remove_char(),

            // FileClipboard
            Action::AddEntryFileClipboard => self.action_add_entry_file_clipboard(),
            Action::RemoveEntryFileClipboard => self.action_remove_entry_file_clipbaord(),
            Action::ClearFileClipboard => self.action_clear_file_clipboard(),
            Action::FocusFileClipboard => self.action_focus_file_clipbaord(),
            Action::ExitFileClipboard => self.action_exit_file_clipboard(),
            _ => {}
        }
    }

    /// Maneja teclas "de texto" cuando el panel
    /// activo espera texto libre, como el panel `Create`.
    pub fn execute_input_key(&mut self, key: KeyCode) {
        match self.active_panel {
            Panel::Create => {
                if let Some(ch) = key.as_char() {
                    self.input.set_char(ch);
                }
            }
            _ => {}
        }
    }

    // ── Acciones: movimiento ────────────────────────────────────────────

    /// Mueve la selección hacia arriba dentro del panel activo.
    /// Si el panel activo es el FilePanel actualiza la metadata selecionada.
    fn action_move_up(&mut self) {
        self.panels[self.active_panel].prev();
        if self.active_panel == Panel::FilePanel {
            self.update_metadata_current();
        }
    }

    /// Mueve la selección hacia abajo dentro del panel activo.
    /// Si el panel activo es el FilePanel actualiza la metadata selecionada.
    fn action_move_down(&mut self) {
        self.panels[self.active_panel].next();
        if self.active_panel == Panel::FilePanel {
            self.update_metadata_current();
        }
    }

    // ── Acciones: navegación y apertura de directorios/archivos ───────

    /// Hace focus al `file_panel`

    fn action_focus_file_panel(&mut self) {
        self.active_panel = Panel::FilePanel;

        if self.explorer.entries.is_empty() {
            self.panels[Panel::FilePanel].reset();
        } else {
            self.panels[Panel::FilePanel].setup(self.explorer.entries.len(), None, None);
        }
    }

    /// Hace focus al `sidebar`
    fn action_focus_sidebar_panel(&mut self) {
        self.active_panel = Panel::SideBar
    }

    /// Va al directorio padre del pwd actual (solo si el FilePanel está activo).
    fn action_go_to_parent_dir(&mut self) {
        if Panel::FilePanel == self.active_panel {
            let path = get_parent_dir(self.explorer.pwd.clone());
            self.open_directory(path);
        }
    }

    /// Navega al path del bookmark seleccionado en el Sidebar y le pasa el
    /// focus al FilePanel.
    pub fn action_open_selected_bookmark(&mut self) {
        if let Some((_, path)) = self.get_current_bookmarks() {
            if let Some(p) = path {
                self.open_directory(p.clone());
                self.active_panel = Panel::FilePanel;
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
            self.pending_external_actions.open_with_app = Some(path);
            return;
        }

        self.open_directory(path);
    }

    /// Si el entry seleccionado es un archivo, lo marca para abrirse en el
    /// editor externo configurado.
    fn action_open_selected_in_editor(&mut self) {
        let Some(entry) = self.get_current_entry() else {
            return;
        };

        if entry.is_file {
            let path = entry.path.clone();
            self.pending_external_actions.open_with_editor = Some(path);
        }
    }

    // ── Acciones: panel Metadata ────────────────────────────────────────

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
        self.active_panel = Panel::FilePanel;
        self.panels[Panel::Metadata].reset();
    }

    // ── Acciones: panel Create ──────────────────────────────────────────

    /// Abre el panel de creación de archivo/directorio, limpiando el input
    /// y mostrando el mensaje inicial del popup.
    fn action_open_create_panel(&mut self) {
        self.active_panel = Panel::Create;
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
            self.active_panel = Panel::FilePanel;
            self.input.clear();
            return;
        }

        let full_path = self.explorer.pwd.join(input_value.clone());

        if let Err(err) = self.create_entry(full_path, &input_value) {
            self.create_state.fail(err);
        } else {
            // Si no hubo error, el panel create se cierra
            self.active_panel = Panel::FilePanel;
            self.create_state.close();
        }

        self.input.clear();
    }

    /// Cancela la creación: cierra el popup, vuelve al FilePanel y
    /// descarta el texto ingresado en el input.
    fn action_cancel_create(&mut self) {
        self.create_state.close();
        self.active_panel = Panel::FilePanel;
        self.input.clear();
    }

    /// Añade el entry selecionado en el file_panel, si el archivo/directorio
    /// existe lo añade a la lista de file_clipboard
    fn action_add_entry_file_clipboard(&mut self) {
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
            self.panels[Panel::FileClipboard].count = self.file_clipboard.len();
            self.panels[Panel::FileClipboard]
                .set_selected(self.file_clipboard.len().saturating_sub(1));
        }
    }

    /// Elimina el entry entry selecionado de la lista del FileClipboard
    fn action_remove_entry_file_clipbaord(&mut self) {
        if self.active_panel != Panel::FileClipboard {
            return;
        }

        self.file_clipboard
            .remove(self.panels[Panel::FileClipboard].selected);

        self.panels[Panel::FileClipboard].clamp_seleted(self.file_clipboard.len());

        if self.file_clipboard.len() == 0 {
            self.action_exit_file_clipboard();
        }
    }

    /// Limpia toda la lista del FileClipboard
    fn action_clear_file_clipboard(&mut self) {
        if self.active_panel != Panel::FileClipboard {
            return;
        }

        self.file_clipboard.clear();

        self.panels[Panel::FileClipboard].clamp_seleted(self.file_clipboard.len());

        self.action_exit_file_clipboard();
    }

    /// Le da focus al FileClipboard
    fn action_focus_file_clipbaord(&mut self) {
        if self.file_clipboard.len() > 0 {
            self.active_panel = Panel::FileClipboard;
            self.panels[Panel::FileClipboard].valid = true;
        }
    }

    // Sale del FileClipboard y le da focus al FilePanel
    fn action_exit_file_clipboard(&mut self) {
        self.active_panel = Panel::FilePanel;
        self.panels[Panel::FileClipboard].valid = false;
    }
}
