use crate::settings;
use std::fs;
use std::collections::HashMap;
use uuid::Uuid;
use std::path::{Path,PathBuf};

use crate::util;
use crate::mods;

pub struct ModpackManager {
    pub packs: HashMap<String, Modpack>, //key is modpack.id
    settings: settings::Settings
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Modpack {
    #[serde(skip_serializing)]
    pub folder_name: Option<String>,

    pub id: Option<String>,
    pub name: String,
    pub author: Option<String>,
    pub versions: ModpackVersionInfo,
    pub settings: PackSettings,
    pub lastPlayed: Option<String>,
    pub created: String,
    pub mods: Vec<mods::SavedModEntry>
}


#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ModpackVersionInfo {
    pub minecraft: String,
    pub modloader: String,
    pub pack: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct PackSettings {
    pub javaMemoryMb: u32,
    pub useCustomMemory: bool,
    pub modloaderType: String,
    pub javaArgs: Option<String>,
    pub modSource: String
}

impl ModpackManager {
    pub fn get_downloads_folder(&self) -> PathBuf {
        Path::new(&self.settings.minecraft.saveDirectory).join("Downloads")
    }


    fn get_instances_folder(&self) -> PathBuf {
        Path::new(&self.settings.minecraft.saveDirectory).join("Instances")
    }

    fn get_install_folder(&self) -> PathBuf {
        Path::new(&self.settings.minecraft.saveDirectory).join("Launcher")
    }

    pub fn new(settings: settings::Settings) -> ModpackManager {
        let manager = ModpackManager {
            packs: HashMap::new(),
            settings 
        };
        manager
    }

    pub fn load(&mut self) {
        let paths = fs::read_dir(self.get_instances_folder()).unwrap();
        for path in paths {
            let entry = path.unwrap();
            if entry.file_type().unwrap().is_dir() {
                let manifest_path = entry.path().join("manifest.json");
                // TODO: Pass invalid or corrupted modpacks to user
                match fs::read_to_string(&manifest_path) {
                    Ok(str) => {
                        match serde_json::from_str::<Modpack>(&str) {
                            Ok(mut modpack) => {
                                let id = modpack.id.as_deref().unwrap().to_string();
                                println!("[debug] Loading modpack uuid = {}", &id);
                                modpack.folder_name = entry.file_name().into_string().ok();
                                self.packs.insert(id, modpack);
                            },
                            Err(err) => {
                                eprintln!("ERROR: Directory \"{}\"'s manifest.json is either incomplete or invalid json: {}", entry.file_name().to_str().unwrap(), err)
                            }
                        }
                        
                    },
                    Err(err) => {
                        eprintln!("ERROR:  Directory \"{}\"'s manifest.json is unreadable or corrupted: {}", entry.file_name().to_str().unwrap(), err)
                    }
                }
            }
        }
    }

    pub fn save(&self, modpack: &Modpack) {
        let folder_name = modpack.folder_name.as_ref().expect("modpack has no save folder");
        let manifest = self.get_instances_folder().join(folder_name).join("manifest.json");
        std::fs::write(manifest, serde_json::to_string_pretty(modpack).unwrap().as_bytes()).expect("save failed");
    }

    pub fn get_modpack_by_name(&self, name: &str) -> Option<&Modpack> {
        for (id, pack) in self.packs.iter() {
            if pack.name == name {
                println!("[debug] Found modpack with id = {}", id);
                return Some(pack)
            }
        }
        None
    }

    pub fn get_modpack(&self, id: &str) -> Option<&Modpack> {
        self.packs.get(id)
    }

    pub fn get_modpack_mut(&mut self, id: &str) -> Option<&mut Modpack> {
        self.packs.get_mut(id)
    }

    pub fn get_modpacks(&mut self) -> Vec<Modpack> {
        self.load();
        self.packs.values()
            .map(|pack| pack.clone())
            .collect::<Vec<Modpack>>()
    }

    pub fn delete_modpack(&mut self, id: &str) -> Option<Modpack> {
        self.packs.remove(id)
    }

    pub fn create_modpack(&mut self, mut pack: Modpack) -> Result<Modpack, String> {
        pack.id = Some(Uuid::new_v4().to_string());
        if self.get_modpack_by_name(pack.name.as_ref()).is_some() {
            let mut found_suitable = false;
            for n in 1..50 {
                let new_name = format!("{} ({})", pack.name, n);
                if self.get_modpack_by_name(&new_name).is_none() {
                    pack.name = new_name;
                    found_suitable = true;
                    break;
                }
            }
            if !found_suitable {
                return Err("Could not create modpack due to duplicates. Why do you have 50?".to_string())
            }
        }

        let save_dir = &self.get_instances_folder().join(&pack.name);
        pack.folder_name = save_dir.clone().into_os_string().into_string().ok();
        std::fs::create_dir_all(save_dir).expect("failed to create modpack folder");
        // Make folders
        std::fs::create_dir_all(save_dir.join("mods")).expect("failed to create modpack/mods folder");
        // Make files
        fs::write(save_dir.join(".mcmm"), pack.id.as_ref().unwrap()).unwrap();
        fs::write(save_dir.join("manifest.json"), serde_json::to_string_pretty(&pack).expect("failed to serialize modpack to manifest")).expect("failed to create modpack manifest");
        println!("[debug] Created new modpack (name = \"{}\") with uuid = {}", &pack.name, pack.id.as_ref().unwrap());
        let id = pack.id.clone().unwrap();
        let clone = pack.clone();
        self.packs.insert(id, pack);
        Ok(clone)
    }

    // Updates the internal settings
    pub fn set_settings(&mut self, settings: settings::Settings) {
        self.settings = settings;
    }

    pub fn launch_modpack(&mut self, id: &str) -> Result<std::process::Child, String> {
        println!("[debug] attempting to launch {}", id);
        match self.get_modpack_mut(id) {
            Some(modpack) => {
                /*
                1. Install launcher to $saveDir/Launcher/
                */
                modpack.lastPlayed = Some(util::get_iso8601(None));
                let modpack = self.get_modpack(id).unwrap();
                self.set_launcher_config(&modpack);
                let work_dir = self.get_install_folder();
                println!("[debug] launching modpack \"{}\" with args: \"-w {}\"", &modpack.name, &work_dir.to_string_lossy());
                match std::process::Command::new(self.get_install_folder().join("MinecraftLauncher.exe"))
                    .arg("-w")
                    .arg(work_dir)
                    .spawn()
                {
                    Ok(child) => Ok(child),
                    Err(err) => Err(err.to_string())
                }
            },
            None => Err("No modpack found with id".to_string())
        }
    }

    #[allow(non_snake_case)]
    fn get_launcher_profile(&self, modpack: &Modpack) -> LauncherProfile {
        let game_dir = self.get_instances_folder().join(&modpack.name).to_str().unwrap().to_owned();
        let java_args = modpack.settings.javaArgs.as_ref()
            .or(self.settings.minecraft.javaArgs.as_ref())
            .or(Some(&"".to_string()))
            .cloned();
        LauncherProfile {
            created: modpack.created.clone(),
            gameDir: Some(game_dir),
            javaArgs: java_args,
            lastUsed: util::get_iso8601(None),
            lastVersionId: "1.16.5".to_string(), //TODO: Use correct jar
            name: modpack.name.clone(),
            resolution: Some(ProfileResolution {
                height: self.settings.minecraft.height,
                width: self.settings.minecraft.width
            }),
            type_: "custom".to_string() 
        }
    }

    pub fn set_launcher_config(&self, modpack: &Modpack) {
        let path = self.get_install_folder().join("launcher_profiles.json");
        let mut profile_config: LauncherProfilesJson = match fs::read_to_string(&path) {
            Ok(profiles) => serde_json::from_str(&profiles).expect("invalid launcher_profiles.json, cannot parse"),
            Err(_) => LauncherProfilesJson {
                profiles: None,
                settings: LauncherSettings::get_default(),
                version: 3
            }
        };
        if profile_config.profiles.is_none() {
            profile_config.profiles = Some(HashMap::new());
        }
        let profiles = profile_config.profiles.as_mut().unwrap();
        profiles.clear();
        profiles.insert(modpack.name.clone(), self.get_launcher_profile(modpack));
        fs::write(&path, serde_json::to_string_pretty(&profile_config).unwrap()).unwrap();
    }

    pub fn add_mod_entry(&mut self, pack_id: &str, entry: mods::SavedModEntry) -> Modpack {
        let mut pack = self.get_modpack(pack_id).expect("add mod entry to not a modpack").clone();
        let download_dir = self.get_downloads_folder();
        let dest = self.get_instances_folder().join(pack.folder_name.as_ref().unwrap()).join("mods");
        std::fs::create_dir_all(&dest).unwrap();
        for filename in &entry.filenames {
            let src_path = download_dir.join(filename);
            let dest_path = dest.join(filename);
            std::fs::rename(src_path, dest_path).expect("failed to move download");
        }
        pack.mods.push(entry);
        self.save(&pack);
        self.packs.insert(pack.id.clone().unwrap(), pack.clone());
        pack
    }

}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct LauncherProfile {
    pub created: String,
    pub gameDir: Option<String>,
    pub javaArgs: Option<String>,
    pub lastUsed: String,
    pub lastVersionId: String,
    pub name: String,
    pub resolution: Option<ProfileResolution>,
    #[serde(rename = "type")]
    pub type_: String
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct LauncherProfilesJson {
    pub profiles: Option<HashMap<String, LauncherProfile>>, //Maybe Option<>
    settings: LauncherSettings,
    version: u8
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct LauncherSettings {
    crashAssistance: bool,
    enableAdvanced: bool,
    enableAnalytics: bool,
    enableHistorical: bool,
    enableReleases: bool,
    enableSnapshots: bool,
    keepLauncherOpen: bool,
    profileSorting: String,
    showGameLog: bool,
    showMenu: bool,
    soundOn: bool
}

impl LauncherSettings {
    pub fn get_default() -> LauncherSettings {
        LauncherSettings {
            crashAssistance: true,
            enableAdvanced: false,
            enableAnalytics: true,
            enableHistorical: false,
            enableReleases: true,
            enableSnapshots: false,
            keepLauncherOpen: false,
            profileSorting: "ByLastPlayed".to_string(),
            showGameLog: false,
            showMenu: false,
            soundOn: false
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ProfileResolution {
    height: u32,
    width: u32
}
