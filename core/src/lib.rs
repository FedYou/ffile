mod config;
pub mod shortcuts;
pub mod watcher;

pub mod actions;
pub mod global;
pub mod open_file;
pub mod types;

pub mod app;
pub mod file_system;
mod index;
mod input;
pub mod processes;

pub use config::{Config, get_config, set_config};

pub fn init() {
    watcher::init_watcher();
}
