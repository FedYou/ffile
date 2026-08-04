use std::{path::PathBuf, process::Command, thread};

use crate::config::get_config;

/// Abre un archivo con la aplicación externa por defecto del sistema
/// (`open::that`) en un hilo aparte para no bloquear el TUI.
pub fn file(path: PathBuf) {
    thread::spawn(move || open::that(path));
}

/// Abre un archivo en el editor externo configurado (`config.editor`),
/// esperando a que el proceso termine.
pub fn editor(path: &str) {
    let config = get_config().lock().unwrap();
    let _ = Command::new(config.editor.clone()).arg(path).status();
}
