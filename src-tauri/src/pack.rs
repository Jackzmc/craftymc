use crate::settings;
use std::fs;
use std::collections::HashMap;
use uuid::Uuid;
use std::path::{Path,PathBuf};
use chrono::{DateTime, Utc};


pub struct ModpackManager {
    pub packs: HashMap<String, Modpack>, //id is folder name
    settings: settings::Settings
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Modpack {
    pub id: Option<String>,
    pub name: String,
    pub versions: ModpackVersionInfo,
    pub settings: PackSettings,
    pub lastPlayed: Option<u32>,
    pub created: u32
    // pub mods: Vec<Mod>
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
    pub javaMemoryMb: Option<u32>,
    pub useCustomMemory: bool,
    pub modloaderType: String,
    pub javaArgs: Option<String>
}

impl ModpackManager {
    fn get_instances_folder(&self) -> PathBuf {
        Path::new(&self.settings.minecraft.saveDirectory).join("Instances")
    }

    fn get_install_folder(&self) -> PathBuf {
        Path::new(&self.settings.minecraft.saveDirectory).join("Launcher")
    }

    pub fn new(settings: settings::Settings) -> ModpackManager {
        let mut manager = ModpackManager {
            packs: HashMap::new(),
            settings 
        };
        manager.load();
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
                            Ok(modpack) => {
                                let id = modpack.id.as_deref().unwrap().to_string();
                                println!("[debug] Loading modpack uuid = {}", &id);
                                self.packs.insert(id, modpack);
                            },
                            Err(err) => {
                                println!("WARN: Directory \"{}\" is either incomplete or invalid json: {}", entry.file_name().to_str().unwrap(), err)
                            }
                        }
                        
                    },
                    Err(err) => {
                        println!("WARN: Directory \"{}\"'s manifest.json is unreadable or corrupted: {}", entry.file_name().to_str().unwrap(), err)
                    }
                }
            }
        }
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

    pub fn get_modpacks(&self) -> Vec<Modpack> {
        self.packs.values()
            .map(|pack| pack.clone())
            .collect::<Vec<Modpack>>()
    }

    pub fn delete_modpack(&mut self, id: &str) -> Option<Modpack> {
        self.packs.remove(id)
    }

    pub fn create_modpack(&mut self, mut pack: Modpack) -> Result<Modpack, String> {
        pack.id = Some(Uuid::new_v4().to_string());
        pack.created = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32;
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

        let save_dir = self.get_instances_folder().join(&pack.name);
        std::fs::create_dir_all(&save_dir).expect("failed to create modpack folder");
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

    pub fn launch_modpack(&self, id: &str) -> Result<std::process::Child, String> {
        println!("[debug] attempting to launch {}", id);
        match self.get_modpack(id) {
            Some(modpack) => {
                /*
                1. Install launcher to $saveDir/Launcher/
                2. Somehow:
                    a) install 'versions'
                    b) create a profile
                        "Ethos Custom Modded (1.16)": {
                            "created": "2022-05-17T23:37:49.271Z",
                            "gameDir": "D:\\Jackz\\Documents\\Curse\\Minecraft\\Instances\\Ethos Modded (1.16)\\",
                            "javaArgs": "-Xmx5984m -Xms256m -Dminecraft.applet.TargetDirectory=\"D:\\Jackz\\Documents\\Curse\\Minecraft\\Instances\\Ethos Modded (1.16)\" -Dfml.ignorePatchDiscrepancies=true -Dfml.ignoreInvalidMinecraftCertificates=true -Duser.language=en -Duser.country=US -XX:UseSSE=3 -XX:+UseG1GC",
                            "lastUsed": "2022-05-17T23:38:46.707Z",
                            "lastVersionId": "forge-36.2.34",
                            "name": "Ethos Custom Modded (1.16)",
                            "resolution": {
                                "height": 768,
                                "width": 1024
                            },
                            "type": "custom"
                        },
                */
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
        let lastUsed = match modpack.lastPlayed {
            Some(time) => DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(time as i64, 0), Utc).to_rfc3339(),
            None => {
               let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
               DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(now, 0), Utc).to_rfc3339()
            }
        };
        let java_args = modpack.settings.javaArgs.as_ref()
            .or(self.settings.minecraft.javaArgs.as_ref())
            .or(Some(&"".to_string()))
            .cloned();
        LauncherProfile {
            created: DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(modpack.created as i64, 0), Utc).to_rfc3339(),
            gameDir: Some(game_dir),
            javaArgs: java_args,
            lastUsed,
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
