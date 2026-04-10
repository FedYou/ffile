mod utils;

use crate::fs::dir::{
    get_dir_entries, get_user_dirs, get_user_home_dir, EntriesOptions, Entry, Sort, UserItem,
};

use std::path::PathBuf;

use utils::IndexController;

pub use utils::IndexAction;

#[derive(PartialEq)]
pub enum Mode {
    SideBar,
    FilePanel,
}

pub struct IndexList {
    pub sidebar: Option<usize>,
    pub file_panel: Option<usize>,
}

pub struct Controller {
    pub current_dir: PathBuf,
    pub user_dir: Vec<UserItem>,
    pub current_entries: Vec<Entry>,
    pub index_list: IndexList,
    pub mode: Mode,
    index: IndexController,
}

impl Controller {
    pub fn new() -> Self {
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

        Self {
            current_dir,
            current_entries,
            user_dir,
            index_list: IndexList {
                sidebar: Some(0),
                file_panel: None,
            },
            mode: Mode::SideBar,
            index: IndexController {
                current: 0,
                len: user_dir_len,
                ignore: vec![],
            },
        }
    }
    fn open_directory(&mut self, path: PathBuf) {
        self.current_dir = path;

        let options = self.get_entries_option();

        let expect_msg = format!(
            "Cannot get contents of {} directory",
            self.current_dir.to_str().unwrap()
        );

        self.current_entries = get_dir_entries(options).expect(&expect_msg);
        self.index_list.file_panel = None
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
                self.index.set_len(self.current_entries.len());
                self.index.set_current(self.index_list.file_panel);
                self.index.apply_action(index_action);
                self.index_list.file_panel = Some(self.index.current)
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
                // Si no esta seleccionado nada seleciona el primer archivo al cambiar
                // Solo si hay items para seleccionar
                if !self.current_entries.is_empty() && (self.index_list.file_panel == None) {
                    self.index_list.file_panel = Some(0)
                }
            }
            _ => {
                self.mode = Mode::SideBar;
            }
        }
    }
}
