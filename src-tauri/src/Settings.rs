use std::path::Path;
use sysinfo::SystemExt;

#[derive(serde::Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Settings {
    general: GeneralSettings,
    minecraft: MinecraftSettings,
    meta: MetaInfo
}

#[derive(serde::Serialize, Clone)]
#[allow(non_snake_case)]
pub struct GeneralSettings {
}

#[derive(serde::Serialize, Clone)]
#[allow(non_snake_case)]
pub struct MinecraftSettings {
    saveDirectory: String,
    preferredRelease: String,
    width: u32,
    height: u32,
    javaMemoryMb: u32,
    javaPath: Option<String>,
    javaArgs: Option<String>
}

#[derive(serde::Serialize, Clone)]
#[allow(non_snake_case)]
pub struct MetaInfo {
    maxMemoryMb: u64
}

impl Settings{
    pub fn get_save_folder() -> std::path::PathBuf {
        Path::new(&dirs_next::document_dir().unwrap()).join("MCModDownloader")
    }

    pub fn new() -> Settings {
        let mut system = sysinfo::System::new();
        system.refresh_all();
        Settings {
            meta: MetaInfo {
                maxMemoryMb: system.total_memory() / 1000
            },
            general: GeneralSettings {

            },
            minecraft: MinecraftSettings {
                saveDirectory: Settings::get_save_folder().to_str().unwrap().to_string(),
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