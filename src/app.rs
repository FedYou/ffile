use std::path::PathBuf;

use arboard::Clipboard;
use notify::{RecommendedWatcher, Watcher};

use crate::config::get_config;
use crate::controller::{IndexAction, IndexController};
use crate::fs::dir::{
    get_dir_entries, get_parent_dir, get_user_dirs, get_user_home_dir, nearest_existing_dir,
    EntriesOptions, Entry, UserItem,
};
use crate::fs::metadata::{serialize_metadata, SerializeMetadata};
use crate::global;
use crate::keyboard::Action;
use crate::tui;

#[derive(PartialEq)]
pub enum Mode {
    SideBar,
    FilePanel,
    Metadata,
}

pub struct IndexList {
    pub sidebar: Option<usize>,
    pub file_panel: Option<usize>,
    pub metadata: Option<usize>,
}

pub struct App<'a> {
    pub title: &'a str,
    pub exit: bool,
    pub is_small: bool,
    pub mode: Mode,
    pub user_dirs: Vec<UserItem>,
    pub current_dir: PathBuf,
    pub current_entries: Vec<Entry>,
    pub current_metadata: Option<Vec<SerializeMetadata>>,
    index: IndexController,
    pub index_list: IndexList,
    watcher: RecommendedWatcher,
    clipboard: Clipboard,
}

impl<'a> App<'a> {
    pub fn new(mut watcher: RecommendedWatcher) -> Self {
        let config = get_config().lock().unwrap();
        let current_dir = get_user_home_dir();
        let user_dirs = get_user_dirs();
        let user_dirs_len = user_dirs.len();

        let dir_entries_options = EntriesOptions {
            path: current_dir.clone(),
            sort: config.sort.clone(),
            invert: config.invert_sort.clone(),
            show_hidden: config.show_hidden_files.clone(),
            filter: None,
        };

        let current_entries =
            get_dir_entries(dir_entries_options).expect("Cannot get contents of HOME directory");

        let file_panel_index: Option<usize> = if !current_entries.is_empty() {
            Some(0)
        } else {
            None
        };

        let current_metadata = get_metadata_current(&current_entries, file_panel_index);

        let _ = watcher.watch(&current_dir, notify::RecursiveMode::NonRecursive);

        return App {
            title: &global::TITLE,
            exit: false,
            is_small: false,
            mode: Mode::SideBar,
            user_dirs,
            current_dir,
            current_entries,
            current_metadata,
            index: IndexController {
                current: 0,
                len: user_dirs_len,
                ignore: vec![],
            },

            index_list: IndexList {
                sidebar: Some(0),
                file_panel: file_panel_index,
                metadata: None,
            },
            watcher,
            clipboard: Clipboard::new().unwrap(),
        };
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        let term_area = frame.area();

        if term_area.width < 80 || term_area.height < 18 {
            self.is_small = true;
        } else {
            self.is_small = false
        }

        tui::draw(frame, self);
    }

    pub fn update(&mut self) {
        self.current_dir = nearest_existing_dir(self.current_dir.clone());

        let options = self.get_entries_option();

        let expect_msg = format!(
            "Cannot get contents of {} directory",
            self.current_dir.to_str().unwrap()
        );

        self.current_entries = get_dir_entries(options).expect(&expect_msg);

        if self.current_entries.is_empty() {
            self.index_list.file_panel = None;
        } else if let Some(index) = self.index_list.file_panel {
            // Selecciona el ultimo elemento si el index es mayor a al cantidad de elementos
            if index > self.current_entries.len() - 1 {
                self.index_list.file_panel = Some(self.current_entries.len() - 1);
            }
        }

        self.fix_open_mode_file_panel();
        self.update_metadata_current();
    }

    pub fn execute_action(&mut self, action: Action) {
        if action == Action::Exit {
            self.exit = true
        }

        if self.is_small {
            return;
        }

        match action {
            Action::Up => {
                self.action_move_up();
            }
            Action::Down => {
                self.action_move_down();
            }
            Action::Open => {
                self.action_open();
            }
            Action::ToParent => {
                self.action_to_parent_directory();
            }
            Action::ChangeModeToggle => {
                self.action_change_mode_toggle();
            }
            Action::ChangeModeMetadata => {
                self.action_change_mode_metadata();
            }
            Action::CopyMetadata => {
                self.action_copy_metadata_selected();
            }
            _ => {}
        }
    }

    // ------------------------------------
    // ------------------------------------
    // ---------- Others functions --------
    // ------------------------------------
    // ------------------------------------

    fn get_entries_option(&self) -> EntriesOptions {
        let config = get_config().lock().unwrap();
        return EntriesOptions {
            path: self.current_dir.clone(),
            sort: config.sort.clone(),
            invert: config.invert_sort.clone(),
            show_hidden: config.show_hidden_files,
            filter: None,
        };
    }

    fn update_metadata_current(&mut self) {
        // Actualiza la lista de metadata actual
        self.current_metadata =
            get_metadata_current(&self.current_entries, self.index_list.file_panel)
    }

    fn fix_open_mode_file_panel(&mut self) {
        // Si hay entries y el index_list no selecciono ninguno entry, va seleccionar el primer elemento
        if !(self.current_entries.is_empty()) && (self.index_list.file_panel == None) {
            self.index_list.file_panel = Some(0)
        }
    }

    // ------------------------------------
    // ------------------------------------
    // ------- Auxiliary functions --------
    // ------------------------------------
    // ------------------------------------

    fn change_index(&mut self, index_action: IndexAction) {
        match self.mode {
            Mode::SideBar => {
                self.index.set_len(self.user_dirs.len());
                self.index.set_current(self.index_list.sidebar);
                self.index.apply_action(index_action);
                self.index_list.sidebar = Some(self.index.current)
            }
            Mode::FilePanel => {
                if self.current_entries.is_empty() {
                    return;
                }
                self.index.set_len(self.current_entries.len());
                self.index.set_current(self.index_list.file_panel);
                self.index.apply_action(index_action);
                self.index_list.file_panel = Some(self.index.current);

                self.update_metadata_current();
            }
            Mode::Metadata => {
                if self.current_metadata == None {
                    return;
                }

                self.index.set_len(
                    self.current_metadata
                        .clone()
                        .expect("Can't get list len from metadata")
                        .len(),
                );
                self.index.set_current(self.index_list.metadata);
                self.index.apply_action(index_action);
                self.index_list.metadata = Some(self.index.current);
            }
        }
    }

    fn open_directory(&mut self, path: PathBuf) {
        let valid_dir = nearest_existing_dir(path);

        let _ = self.watcher.unwatch(&self.current_dir);
        let _ = self
            .watcher
            .watch(&valid_dir, notify::RecursiveMode::NonRecursive);

        self.current_dir = valid_dir;

        let options = self.get_entries_option();

        let expect_msg = format!(
            "Cannot get contents of {} directory",
            self.current_dir.to_str().unwrap()
        );

        self.current_entries = get_dir_entries(options).expect(&expect_msg);
        self.index_list.file_panel = None;
        self.fix_open_mode_file_panel();
        self.update_metadata_current();
    }

    pub fn open_from_sidebar(&mut self) {
        if let Some(index) = self.index_list.sidebar {
            let item = self.user_dirs.get(index).unwrap();
            if let Some(path) = item.path.clone() {
                self.open_directory(path);
                self.mode = Mode::FilePanel;
            }
        }
    }

    fn open_from_file_panel(&mut self) {
        if let Some(index) = self.index_list.file_panel {
            let entry = self.current_entries.get(index).unwrap();

            if entry.is_file {
                let path = entry.path.clone();

                std::thread::spawn(move || match open::that(path) {
                    _ => {}
                });
                return;
            }

            self.open_directory(entry.path.clone());
        }
    }

    // ------------------------------------
    // ------------------------------------
    // ------------- Actions --------------
    // ------------------------------------
    // ------------------------------------

    fn action_move_up(&mut self) {
        self.change_index(IndexAction::Decrease);
    }

    fn action_move_down(&mut self) {
        self.change_index(IndexAction::Increase);
    }

    fn action_to_parent_directory(&mut self) {
        if Mode::FilePanel == self.mode {
            let path = get_parent_dir(self.current_dir.clone());
            self.open_directory(path);
        }
    }

    fn action_open(&mut self) {
        match self.mode {
            Mode::SideBar => {
                self.open_from_sidebar();
            }
            Mode::FilePanel => {
                self.open_from_file_panel();
            }
            _ => {}
        }
    }

    fn action_change_mode_toggle(&mut self) {
        match self.mode {
            Mode::SideBar => {
                self.mode = Mode::FilePanel;
            }
            Mode::FilePanel => {
                self.mode = Mode::SideBar;
            }
            Mode::Metadata => {
                self.mode = Mode::FilePanel;
                self.index_list.metadata = None
            }
        }
    }

    fn action_change_mode_metadata(&mut self) {
        if self.mode == Mode::Metadata {
            return;
        }

        self.mode = Mode::Metadata;
        self.index_list.metadata = Some(0)
    }

    fn action_copy_metadata_selected(&mut self) {
        if self.mode != Mode::Metadata {
            return;
        }

        if let Some(metadata) = &self.current_metadata {
            if let Some(index) = self.index_list.metadata {
                let value = metadata.get(index).unwrap().value.as_str();
                self.clipboard.set_text(value).unwrap();
            }
        }
    }
}

fn get_metadata_current(
    current_entries: &Vec<Entry>,
    index: Option<usize>,
) -> Option<Vec<SerializeMetadata>> {
    if let Some(index) = index {
        if current_entries.is_empty() {
            return None;
        }
        return Some(serialize_metadata(&current_entries.get(index).unwrap()));
    }
    None
}
