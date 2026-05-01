use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::{fs, sync::OnceLock};

use crate::fs::dir::{get_config_dir, Sort};

pub const CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| get_config_dir().to_path_buf());

pub const CONFIG_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| CONFIG_PATH.clone().join("config.toml").to_path_buf());

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default = "default_sort")]
    pub sort: Sort,
    #[serde(default = "default_invert_sort")]
    pub invert_sort: bool,
    #[serde(default = "default_show_hidden_files")]
    pub show_hidden_files: bool,
    #[serde(default = "default_icons")]
    pub icons: bool,
}

fn default_sort() -> Sort {
    Sort::Date
}

fn default_invert_sort() -> bool {
    true
}

fn default_show_hidden_files() -> bool {
    false
}

fn default_icons() -> bool {
    true
}

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

pub fn get_config() -> &'static Mutex<Config> {
    return CONFIG.get_or_init(|| Mutex::new(get_config_file()));
}

fn get_config_file() -> Config {
    if let Ok(str) = fs::read_to_string(&*CONFIG_FILE_PATH) {
        return toml::from_str(&str).expect("Invalid config");
    }

    let default_config: Config = get_default_config();

    let file_content = toml::to_string_pretty(&default_config).unwrap();

    fs::create_dir_all(&*CONFIG_PATH).unwrap();

    fs::write(&*CONFIG_FILE_PATH, &file_content).unwrap();

    return default_config;
}

fn get_default_config() -> Config {
    // Añade los valores default, no da error por que es un autocompletado
    // Solo da error a compos desconcidos
    toml::from_str("").unwrap()
}

pub fn update_config_file() {
    let config = get_config().lock().unwrap();

    let file_content = toml::to_string_pretty(&*config).unwrap();

    fs::create_dir_all(&*CONFIG_PATH).unwrap();

    fs::write(&*CONFIG_FILE_PATH, &file_content).unwrap();
}
