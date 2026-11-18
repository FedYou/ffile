use crate::{
    config::config::{CONFIG, Config},
    global::{CONFIG_DIR, CONFIG_PATH},
};
use std::{
    fs,
    sync::{Mutex, MutexGuard},
};

pub fn get_config() -> MutexGuard<'static, Config> {
    return CONFIG.get_or_init(|| Mutex::new(load())).lock().unwrap();
}

pub fn set_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(&*CONFIG_DIR)?;

    let content = toml::to_string_pretty(config)?;

    fs::write(&*CONFIG_PATH, content)?;

    *get_config() = config.clone();

    Ok(())
}

fn load() -> Config {
    if let Ok(str) = fs::read_to_string(&*CONFIG_PATH) {
        return toml::from_str(&str).expect("Invalid config");
    }

    let default_config: Config = Config::default();
    let content = toml::to_string_pretty(&default_config).unwrap();

    fs::create_dir_all(&*CONFIG_DIR).unwrap();

    fs::write(&*CONFIG_PATH, &content).unwrap();

    return default_config;
}
