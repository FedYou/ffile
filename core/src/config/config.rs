use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

use crate::file_system::directories::Sort;
use crate::shortcuts::Shortcuts;

pub static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub sort: Sort,
    pub invert_sort: bool,
    pub show_hidden_files: bool,
    pub file_icons: bool,
    pub editor: String,
    pub shortcuts: Shortcuts,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            sort: Sort::Date,
            invert_sort: true,
            show_hidden_files: false,
            file_icons: true,
            editor: "helix".to_string(),
            shortcuts: Shortcuts::default(),
        }
    }
}
