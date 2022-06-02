use std::path::Path;
use sysinfo::SystemExt;
use std::fs;
#[allow(unused_imports)]
use log::{info, debug, error, warn};

pub struct SettingsManager {
    pub settings: Settings,
    file_path: std::path::PathBuf
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Settings {
    pub general: GeneralSettings,
    pub minecraft: MinecraftSettings,
    pub meta: Option<MetaInfo>
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct GeneralSettings {
    pub telemetryState: i8
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct MinecraftSettings {
    pub saveDirectory: String,
    pub preferredRelease: String,
    pub width: u32,
    pub height: u32,
    pub javaMemoryMb: u32,
    pub javaPath: Option<String>,
    pub javaArgs: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct MetaInfo {
    pub maxMemoryMb: u64,
    pub appVersion: Option<String>
}

impl Settings {
    pub fn get_default(save_dir: &std::path::PathBuf) -> Settings {
        Settings {
            meta: None,
            general: GeneralSettings {
                telemetryState: -1
            },
            minecraft: MinecraftSettings {
                saveDirectory: save_dir.to_str().unwrap().to_string(),
                preferredRelease: "release".to_string(),
                width: 1920,
                height: 1080,
                javaMemoryMb: 1000,
                javaPath: None,
                javaArgs: None
            }
        }
    }
}



impl SettingsManager {
    pub fn get_save_folder() -> std::path::PathBuf {
        Path::new(&dirs_next::document_dir().unwrap()).join("CraftyMc")
    }

    pub fn new() -> SettingsManager {
        let save_dir = SettingsManager::get_save_folder();
        for folder in ["Instances", "Downloads", "Launcher", "Exports"] {
            std::fs::create_dir_all(save_dir.join(folder)).unwrap();
        }

        let config_file_path = save_dir.join("settings.json");
        let settings = SettingsManager::load(&save_dir);
        
        SettingsManager {
            settings,
            file_path: config_file_path
        }
    }

    pub fn load(save_dir: &std::path::PathBuf) -> Settings {
        let config_file_path = Path::new(save_dir).join("settings.json");
        let mut settings = match fs::read_to_string(&config_file_path) {
            Ok(str) => {
                serde_json::from_str(&str).unwrap()
            },
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    warn!("Error reading settings file, resetting to default. {}", err);
                }
                Settings::get_default(save_dir)
            }
        };
        // Initalize any system info needed:
        let mut system = sysinfo::System::new();
        system.refresh_all();
        settings.meta = Some(MetaInfo {
            maxMemoryMb: system.total_memory() / 1000,
            appVersion: None,
        });
        settings
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let mut copy = self.settings.clone();
        // Don't save meta info
        copy.meta = None;
        let json_str = serde_json::to_string_pretty(&copy).unwrap();
        fs::write(&self.file_path, json_str)
    }

    pub fn set_version(&mut self, version: String) {
        self.settings.meta.as_mut().unwrap().appVersion = Some(version);
    }
}