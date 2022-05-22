use std::path::Path;
use sysinfo::SystemExt;
use std::fs;



#[allow(non_snake_case)]
pub struct SettingsManager {
    pub Settings: Settings,
    file_path: std::path::PathBuf
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Settings {
    pub general: GeneralSettings,
    pub minecraft: MinecraftSettings,
    meta: MetaInfo
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
    maxMemoryMb: u64
}

impl Settings {
    pub fn get_default(save_dir: &std::path::PathBuf) -> Settings {
        let mut system = sysinfo::System::new();
        system.refresh_all();
        Settings {
            meta: MetaInfo {
                maxMemoryMb: system.total_memory() / 1000,
            },
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
        Path::new(&dirs_next::document_dir().unwrap()).join("MCModDownloader")
    }

    pub fn new() -> SettingsManager {
        let save_dir = SettingsManager::get_save_folder();
        std::fs::create_dir_all(&save_dir).unwrap(); //TODO: Send telemetry when created
        std::fs::create_dir_all(&save_dir.join("Instances")).unwrap();
        std::fs::create_dir_all(&save_dir.join("Downloads")).unwrap();
        std::fs::create_dir_all(&save_dir.join("Launcher")).unwrap();
        
        let config_file_path = Path::new(&save_dir).join("settings.json");
        let settings = SettingsManager::load(&save_dir);
        
        SettingsManager {
            Settings: settings,
            file_path: config_file_path
        }
    }

    pub fn load(save_dir: &std::path::PathBuf) -> Settings {
        let config_file_path = Path::new(save_dir).join("settings.json");
        match fs::read_to_string(&config_file_path) {
            Ok(str) => {
                serde_json::from_str(&str).unwrap()
            },
            Err(_) => {
                Settings::get_default(save_dir)
            }
        }
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let json_str = serde_json::to_string_pretty(&self.Settings).unwrap();
        fs::write(&self.file_path, json_str)
    }
}