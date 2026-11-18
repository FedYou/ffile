use directories;
use std::path::PathBuf;
use std::sync::LazyLock;

pub const APP_TITLE: &str = "FFile";

pub const CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let proj =
        directories::ProjectDirs::from("com", "fedyou", "ffile").expect("ProjectDirs failed");

    proj.config_dir().to_path_buf()
});

pub const CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| CONFIG_DIR.clone().join("config.toml"));

/// Tamaño mínimo de la terminal para que la app funcione bien.
pub const MIN_TERM_WIDTH: u16 = 80;
pub const MIN_TERM_HEIGHT: u16 = 18;
