mod utils;

use crate::fs::{
    dir::{
        get_dir_entries, get_parent, get_user_dirs, get_user_home_dir, nearest_existing_dir,
        EntriesOptions, Entry, Sort, UserItem,
    },
    metadata::{serialize_metadata, SerializeMetadata},
};

use arboard::Clipboard;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use utils::IndexController;

pub use utils::IndexAction;

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

pub struct Controller {
    pub current_dir: PathBuf,
    pub user_dir: Vec<UserItem>,
    pub current_entries: Vec<Entry>,
    pub current_metadata: Option<Vec<SerializeMetadata>>,
    pub index_list: IndexList,
    pub mode: Mode,
    pub watcher: RecommendedWatcher,
    index: IndexController,
    clipboard: Clipboard,
}

impl Controller {
    pub fn new(mut watcher: RecommendedWatcher) -> Self {
        let current_dir = get_user_home_dir();
        let user_dir = get_user_dirs();
        let user_dir_len = user_dir.len();
        let options = EntriesOptions {
            path: current_dir.clone(),
            sort: Sort::Date,
            invert: true,
            show_hidden: false,
            filter: None,
        };

        let current_entries =
            get_dir_entries(options).expect("Cannot get contents of HOME directory");

        let file_panel: Option<usize> = if !current_entries.is_empty() {
            Some(0)
        } else {
            None
        };

        let current_metadata = get_metadata_current(&current_entries, file_panel);

        let _ = watcher.watch(&current_dir, RecursiveMode::NonRecursive);

        Self {
            watcher,
            current_dir,
            current_entries,
            current_metadata,
            user_dir,
            index_list: IndexList {
                sidebar: Some(0),
                file_panel: file_panel,
                metadata: None,
            },
            mode: Mode::SideBar,
            index: IndexController {
                current: 0,
                len: user_dir_len,
                ignore: vec![],
            },
            clipboard: Clipboard::new().unwrap(),
        }
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
            if index > self.current_entries.len() - 1 {
                self.index_list.file_panel = Some(self.current_entries.len() - 1);
            }
        }

        self.fix_open_mode_file_panel();
        self.update_metadata_current();
    }

    fn update_metadata_current(&mut self) {
        self.current_metadata =
            get_metadata_current(&self.current_entries, self.index_list.file_panel)
    }

    fn open_directory(&mut self, path: PathBuf) {
        let valid_dir = nearest_existing_dir(path);

        let _ = self.watcher.unwatch(&self.current_dir);
        let _ = self.watcher.watch(&valid_dir, RecursiveMode::NonRecursive);

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
            let item = self.user_dir.get(index).unwrap();

            self.open_directory(item.path.clone().expect("REASON"));
            self.mode = Mode::FilePanel;
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

    pub fn open(&mut self) {
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

    pub fn to_parent_directory(&mut self) {
        if Mode::FilePanel == self.mode {
            let path = get_parent(self.current_dir.clone());
            self.open_directory(path);
        }
    }

    fn get_entries_option(&self) -> EntriesOptions {
        return EntriesOptions {
            path: self.current_dir.clone(),
            sort: Sort::Date,
            invert: true,
            show_hidden: false,
            filter: None,
        };
    }

    fn change_index(&mut self, index_action: IndexAction) {
        match self.mode {
            Mode::SideBar => {
                self.index.set_len(self.user_dir.len());
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

    pub fn change_index_vertical(&mut self, index_action: IndexAction) {
        self.change_index(index_action);
    }

    pub fn change_mode_toggle(&mut self) {
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

    pub fn change_mode_metadata(&mut self) {
        if self.mode == Mode::Metadata {
            return;
        }

        self.mode = Mode::Metadata;
        self.index_list.metadata = Some(0)
    }

    pub fn copy_metadata_selected(&mut self) {
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

    fn fix_open_mode_file_panel(&mut self) {
        // Si no esta seleccionado nada seleciona el primer archivo al cambiar
        // Solo si hay items para seleccionar
        if !(self.current_entries.is_empty()) && (self.index_list.file_panel == None) {
            self.index_list.file_panel = Some(0)
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
