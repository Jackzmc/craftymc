use crate::settings;
use std::fs;
use std::collections::HashMap;
use uuid::Uuid;
use std::path::{Path,PathBuf};


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
    pub modloaderType: String
}

impl ModpackManager {
    fn get_instances_folder(&self) -> PathBuf {
        Path::new(&self.settings.minecraft.saveDirectory).join("Instances")
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
                match fs::read_to_string(&manifest_path) {
                    Ok(str) => {
                        let modpack: Modpack = serde_json::from_str(&str).unwrap();
                        let id = modpack.id.as_deref().unwrap().to_string();
                        println!("[debug] Loading modpack uuid = {}", &id);
                        self.packs.insert(id, modpack);
                    },
                    Err(err) => {
                        println!("WARN: Directory \"{}\" does not have a valid manifest.json file. {}", entry.file_name().to_str().unwrap(), err)
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
        std::fs::create_dir_all(&save_dir).unwrap();
        fs::write(save_dir.join(".mcmm"), pack.id.as_ref().unwrap()).unwrap();
        fs::write(save_dir.join("manifest.json"), serde_json::to_string_pretty(&pack).unwrap()).unwrap();
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

    pub fn launch_modpack(&self, id: &str) -> Result<(), String> {
        println!("[debug] attempting to launch {}", id);
        match self.get_modpack(id) {
            Some(modpack) => {
                /*
                1. Install launcher to $saveDir/Launcher/
                2. Somehow :
                    a) install 'versions'
                    b) create a profile
                        "Ethos Custom Modded (1.16)" : {
                            "created" : "2022-05-17T23:37:49.271Z",
                            "gameDir" : "D:\\Jackz\\Documents\\Curse\\Minecraft\\Instances\\Ethos Modded (1.16)\\",
                            "javaArgs" : "-Xmx5984m -Xms256m -Dminecraft.applet.TargetDirectory=\"D:\\Jackz\\Documents\\Curse\\Minecraft\\Instances\\Ethos Modded (1.16)\" -Dfml.ignorePatchDiscrepancies=true -Dfml.ignoreInvalidMinecraftCertificates=true -Duser.language=en -Duser.country=US -XX:UseSSE=3 -XX:+UseG1GC",
                            "lastUsed" : "2022-05-17T23:38:46.707Z",
                            "lastVersionId" : "forge-36.2.34",
                            "name" : "Ethos Custom Modded (1.16)",
                            "resolution" : {
                                "height" : 768,
                                "width" : 1024
                            },
                            "type" : "custom"
                        },
                */
                let work_dir = self.get_instances_folder().join(&modpack.name).as_os_str().to_owned();
                println!("[debug] launching \"{}\" with args: \"-w {}\"", &modpack.name, &work_dir.to_string_lossy());
                let result = std::process::Command::new("D:\\Jackz\\Documents\\MCModDownloader\\Launcher\\MinecraftLauncher.exe")
                    .arg("-l")
                    .arg(work_dir)
                    .output()
                    .expect("failed to execute process");
                println!("prgm exit code {}.\nstdout: {}\nstderr: {}",
                    result.status,
                    String::from_utf8(result.stdout).unwrap(),
                    String::from_utf8(result.stderr).unwrap(),
                );
                Ok(())
            },
            None => Err("No modpack found with id".to_string())
        }
    }
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct LauncherProfile {
    created: String,
    gameDir: String,
    javaArgs: String,
    lastUsed: String,
    lastVersionId: String,
    name: String,
    resolution: ProfileResolution,
    #[serde(rename = "type")]
    type_: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ProfileResolution {
    height: u32,
    width: u32
}