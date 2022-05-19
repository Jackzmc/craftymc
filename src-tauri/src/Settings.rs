use std::path::Path;
use sysinfo::SystemExt;
use std::fs::File;

#[allow(non_snake_case)]
pub struct SettingsManager {
    pub Settings: Settings,
    pub config_file: File
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
    pub fn new(save_dir: &std::path::PathBuf) -> Settings {
        let mut system = sysinfo::System::new();
        system.refresh_all();
        Settings {
            meta: MetaInfo {
                maxMemoryMb: system.total_memory() / 1000,
            },
            general: GeneralSettings {

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
        std::fs::create_dir_all(&save_dir).unwrap();
        let file = File::create(Path::new(&save_dir).join("settings.json")).unwrap();
        
        SettingsManager {
            Settings: Settings::new(&save_dir),
            config_file: file
        }
    }
}