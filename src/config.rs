use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub theme: String,
    pub volume: f32,
    pub last_folder: Option<String>,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            theme: "Dark".to_string(),
            volume: 0.8,
            last_folder: None,
            window_width: 1000.0,
            window_height: 700.0,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let mut path = if let Some(config_dir) = dirs::config_dir() {
            config_dir
        } else {
            PathBuf::from(".")
        };
        path.push("rustmusic");
        fs::create_dir_all(&path).ok();
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => {
                        return config;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse config: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read config: {}", e);
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::config_path();
        match toml::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(&path, content) {
                    eprintln!("Failed to save config: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize config: {}", e);
            }
        }
    }
}