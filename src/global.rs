use std::path::PathBuf;
use std::sync::LazyLock;

use crate::fs::dir::get_config_dir;

pub const TITLE: &str = "FFile";

pub const CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| get_config_dir().to_path_buf());

pub const CONFIG_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| CONFIG_PATH.clone().join("config.toml").to_path_buf());

pub const MIN_WIDTH_APP: u16 = 80;
pub const MIN_HEIGHT_APP: u16 = 18;
