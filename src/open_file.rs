use std::{path::PathBuf, process::Command, thread};

use crate::config::get_config;

pub fn file(path: PathBuf) {
    thread::spawn(move || open::that(path));
}

pub fn editor(path: &str) {
    let config = get_config().lock().unwrap();
    let _ = Command::new(config.editor.clone()).arg(path).status();
}
