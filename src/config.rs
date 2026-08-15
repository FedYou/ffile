use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{fs, sync::OnceLock};

use crate::file_system::directories::Sort;
use crate::global::{CONFIG_FILE_PATH, CONFIG_PATH};

/// Config global: se carga una única vez y queda disponible para toda la app.
static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

// Config

/// Configuración del explorador, serializada desde el archivo
/// `config.toml`. Todos los campos tienen un valor por defecto, así que
/// el archivo puede omitir cualquier opción.
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
    #[serde(default = "default_editor")]
    pub editor: String,
}

/// Orden de los archivos por defecto: por fecha de modificación.
fn default_sort() -> Sort {
    Sort::Date
}

/// Orden invertido por defecto: los archivos más nuevos primero.
fn default_invert_sort() -> bool {
    true
}

/// Ocultar archivos ocultos (que empiezan con `.`) por defecto.
fn default_show_hidden_files() -> bool {
    false
}

/// Mostrar iconos por defecto.
fn default_icons() -> bool {
    true
}

/// Editor por defecto: helix.
fn default_editor() -> String {
    "helix".to_string()
}

// Funciones

/// Devuelve la configuración global, cargándola una sola vez (la primera
/// llamada) y guardándola en un `OnceLock`. Siempre se accede con lock.
pub fn get_config() -> &'static Mutex<Config> {
    return CONFIG.get_or_init(|| Mutex::new(get_config_file()));
}

/// Carga la config desde el archivo `config.toml`. Si el archivo no existe
/// (o no se puede leer), crea uno nuevo con los valores por defecto.
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

/// Genera una `Config` con todos los valores por defecto. Parsea un TOML
/// vacío: como cada campo tiene su `#[serde(default = "...")]`, no da error.
fn get_default_config() -> Config {
    toml::from_str("").unwrap()
}

/// Vuelve a escribir la configuración actual en `config.toml`, usando los
/// valores que estén en memoria. Se usa cuando el usuario cambia una opción.
pub fn update_config_file() {
    let config = get_config().lock().unwrap();

    let file_content = toml::to_string_pretty(&*config).unwrap();

    fs::create_dir_all(&*CONFIG_PATH).unwrap();

    fs::write(&*CONFIG_FILE_PATH, &file_content).unwrap();
}
