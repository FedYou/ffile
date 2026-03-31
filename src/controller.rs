use crate::fs::dir::{
    get_dir_entries, get_user_dirs, get_user_home_dir, EntriesOptions, Entry, Sort, UserItem,
};
use std::path::PathBuf;

pub struct Controller {
    current_dir: PathBuf,
    user_dir: Vec<UserItem>,
    current_entries: Vec<Entry>,
}

impl Controller {
    pub fn new() -> Self {
        let current_dir = get_user_home_dir();

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
            user_dir: get_user_dirs(),
        }
    }
    fn open_directory(&mut self, path: PathBuf) {
        self.current_dir = path;

        let options = self.get_entries_option();

        let expect_msg = format!(
            "Cannot get contents of {} directory",
            self.current_dir.to_str().unwrap()
        );

        self.current_entries = get_dir_entries(options).expect(&expect_msg)
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

    pub fn get_entries(&self) -> Vec<Entry> {
        return self.current_entries.clone();
    }
    pub fn get_user_dirs(&self) -> Vec<UserItem> {
        return self.user_dir.clone();
    }
}
