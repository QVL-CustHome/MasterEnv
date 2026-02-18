use config::{Config, File};
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct AppSettings {
    config_files: Vec<String>,
    ignored_directories: Vec<String>,
}

static SETTINGS: OnceLock<AppSettings> = OnceLock::new();

pub struct Configuration;

impl Configuration {
    fn get_instance() -> &'static AppSettings {
        SETTINGS.get_or_init(|| {
            Config::builder()
                .add_source(File::with_name("../app-config"))
                .build()
                .expect("Configuration build failed")
                .try_deserialize()
                .expect("Configuration deserialization failed")
        })
    }

    fn config_files() -> &'static [String] {
        &Self::get_instance().config_files
    }

    fn ignored_directories() -> &'static [String] {
        &Self::get_instance().ignored_directories
    }

    pub fn is_config_file(path: &Path) -> bool {
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => return false,
        };

        Self::config_files()
            .iter()
            .any(|config_file| file_name.ends_with(config_file))
    }

    pub fn is_ignored(dir_name: &str) -> bool {
        Self::ignored_directories().iter().any(|ignored_dir| ignored_dir == dir_name)
    }
}