use std::path::PathBuf;
use std::sync::LazyLock;

use crate::file_system::directories::get_config_dir;

/// Título que se muestra en la app.
pub const TITLE: &str = "FFile";

/// Directorio de configuración del usuario (depende del SO).
pub const CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| get_config_dir().to_path_buf());

/// Ruta completa al archivo de configuración.
pub const CONFIG_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| CONFIG_PATH.clone().join("config.toml").to_path_buf());

/// Ancho mínimo de la terminal para que la app funcione bien.
pub const MIN_WIDTH_APP: u16 = 80;

/// Alto mínimo de la terminal para que la app funcione bien.
pub const MIN_HEIGHT_APP: u16 = 18;
