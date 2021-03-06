use crate::settings;
use std::fs;
use std::collections::HashMap;
use uuid::Uuid;
use std::path::{Path,PathBuf};
use log::{info, debug, error, warn};
use std::io::{Read, Write};

use crate::types::modrinth::modpacks;
use crate::util;
use crate::mods;
use crate::payloads;

pub struct ModpackManager {
    pub packs: HashMap<String, Modpack>, //key is modpack.id
    settings: settings::Settings,
    pub root_folder: PathBuf,
    window: Option<tauri::Window>, // Set after provide_window
    pub modrinth_manager: Option<modpacks::ModrinthModpackManager>,
    setup: Option<crate::setup::Setup>
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Modpack {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_ext: Option<String>,

    pub id: Option<String>,
    pub name: String,
    pub author: Option<String>,
    pub versions: ModpackVersionInfo,
    pub settings: PackSettings,
    pub lastPlayed: Option<String>,
    pub timesPlayed: Option<u32>,
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
        self.root_folder.join("Downloads")
    }
    pub fn get_instances_folder(&self) -> PathBuf {
        self.root_folder.join("Instances")
    }

    pub fn get_install_folder(&self) -> PathBuf {
        self.root_folder.join("Launcher")
    }

    pub fn new(settings: settings::Settings) -> ModpackManager {
        let mut manager = ModpackManager {
            packs: HashMap::new(),
            root_folder: Path::new(&settings.minecraft.saveDirectory).to_path_buf(),
            settings,
            window: None,
            modrinth_manager: None,
            setup: None
        };
        manager.setup = Some(crate::setup::Setup::new(&manager));
        manager
    }
    pub fn provide_window(&mut self, window: tauri::Window) {
        self.modrinth_manager = Some(modpacks::ModrinthModpackManager::new(window.clone()));
        self.window = Some(window);
    }

    fn load_entry(&mut self, entry: &std::path::Path) -> Result<Modpack, String> {
        let manifest_path = entry.join("manifest.json");
        // TODO: Pass invalid or corrupted modpacks to user
        let filename = entry.file_name().unwrap().to_str().unwrap();
        let mut modpack = match fs::read_to_string(&manifest_path) {
            Ok(str) => {
            match serde_json::from_str::<Modpack>(&str) {
                Ok(mut modpack) => {
                    if let Some(id) = modpack.id.as_ref() {
                        debug!("loading modpack id = {} in \"{}\"", &id, &filename);
                    } else {
                        debug!("loading modpack id = NONE in \"{}\"", &filename);
                    }
                    modpack.img_ext = self.get_pack_img_ext(&filename);
                    modpack.folder_name = Some(filename.to_string());
                    modpack
                },
                Err(err) => {
                    let err = format!("Directory \"{}\"'s manifest.json is either incomplete or invalid json: {}", filename, err);
                    warn!("{}", &err);
                    return Err(err)
                }
            }
            },
            Err(err) => {
                let error = match err.kind() {
                    std::io::ErrorKind::NotFound => format!("\"{}\" is missing a manifest.json", filename),
                    std::io::ErrorKind::PermissionDenied => format!("Cannot read \"{}\"/manifest.json: Permission Denied", filename),
                    _ => format!("Error reading \"{}\"'s manifest.json: {}", filename, err)
                };
                warn!("{}", &error);
                return Err(error)
            }
        };
        let mods_dir = entry.join("mods");
        if mods_dir.exists() && mods_dir.is_dir() {
            let mut mods = Vec::new();
            for mod_entry in fs::read_dir(&mods_dir).unwrap() {
                let mod_entry = mod_entry.unwrap();
                let mod_path = mod_entry.path();
                let filename = mod_path.file_name().unwrap().to_str().unwrap();
                if !modpack.mods.iter().any(|x| &x.filename == &filename) {
                    let mod_entry = crate::mods::SavedModEntry {
                        project_id: None,
                        version_id: None,
                        name: None,
                        author: None,
                        filename: filename.to_string(),
                        sha512: None,
                        sha1: None,
                    };
                    mods.push(mod_entry);
                }
            }
            modpack.mods.append(&mut mods);
        }
        return Ok(modpack)
    }

    pub fn load(&mut self) {
        let paths = fs::read_dir(self.get_instances_folder()).expect("cant find Instances folder");
        self.packs.clear();
        let window = self.window.as_ref().cloned().unwrap();
        for path in paths {
            let entry = path.unwrap();
            if entry.file_type().unwrap().is_dir() {
                match self.load_entry(&entry.path()) {
                    Ok(modpack) => {
                        self.packs.insert(modpack.id.as_ref().unwrap().to_string(), modpack);
                    },
                    Err(err) => {
                        let filename = entry.file_name().into_string().unwrap();
                        debug!("emitting event[update-modpack] with state Invalid");
                        window.emit("update-modpack", payloads::UpdateModpackPayload {
                            modpack: None,
                            data: None,
                            state: payloads::UpdateModpackState::Invalid(filename, err)
                        }).unwrap();
                    }
                }
            }
        }
    }

    fn get_pack_img_ext(&self, folder_name: &str) -> Option<String> {
        let root = self.get_instances_folder().join(folder_name);
        if root.join("pack.png").exists() {
            Some("png".to_string())
        } else if root.join("pack.jpg").exists() {
            Some("jpg".to_string())
        } else if root.join("pack.webp").exists() {
            Some("webp".to_string())
        } else {
            None
        }
    }

    pub fn save(&self, modpack: &Modpack) {
        let folder_name = modpack.folder_name.as_ref().expect("modpack has no save folder");
        let manifest = self.get_instances_folder().join(folder_name).join("manifest.json");
        let mut modpack = modpack.clone();
        modpack.folder_name = None;
        modpack.img_ext = None;
        std::fs::write(manifest, serde_json::to_string_pretty(&modpack).unwrap().as_bytes()).expect("save failed");
    }

    pub fn get_modpack_by_name(&self, name: &str) -> Option<&Modpack> {
        for (_id, pack) in self.packs.iter() {
            if pack.name == name {
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

    pub fn replace(&mut self, modpack: Modpack) {
        let id = modpack.id.as_ref().unwrap();
        self.packs.insert(id.clone(), modpack);
    }

    pub fn delete_modpack(&mut self, id: &str) -> Result<Option<Modpack>, String> {
      info!("removed modpack id = {}", id);
      if let Some(pack) = self.packs.remove(id) {
        let folder_name = pack.folder_name.as_ref().unwrap();
        match std::fs::remove_dir_all(self.get_instances_folder().join(folder_name)) {
          Ok(_) => Ok(Some(pack)),
          Err(err) => {
            error!("Error deleting modpack \"{}\": {}", folder_name , &err);
            return Err(format!("Could not delete modpack folder \"{}\": {}", folder_name, err).to_string());
          }
        }
      } else {
        return Ok(None)
      }
    }

    pub fn get_suitable_name(&self, name: &str) -> Option<String> {
        let mut new_name = name.to_string();
        let instances_dir = self.get_instances_folder();
        if self.get_modpack_by_name(name).is_some() || instances_dir.join(&name).exists() {
            for n in 1..50 {
                new_name = format!("{} ({})", name, n);
                if self.get_modpack_by_name(&new_name).is_none() && !instances_dir.join(&new_name).exists() {
                    return Some(new_name);
                }
            }
            return None
        }
        Some(new_name)
    }

    pub fn create_modpack(&mut self, mut pack: Modpack) -> Result<Modpack, String> {
        pack.id = Some(Uuid::new_v4().to_string());
        pack.name = self.get_suitable_name(pack.name.as_ref()).expect("Could not create modpack due to duplicates. Why do you have 50 duplicates?");

        let save_dir = &self.get_instances_folder().join(&pack.name);
        pack.folder_name = save_dir.clone().into_os_string().into_string().ok();
        std::fs::create_dir_all(save_dir).expect("failed to create modpack folder");
        // Make folders
        std::fs::create_dir_all(save_dir.join("mods")).expect("failed to create modpack/mods folder");
        // Make files
        fs::write(save_dir.join("manifest.json"), serde_json::to_string_pretty(&pack).expect("failed to serialize modpack to manifest")).expect("failed to create modpack manifest");
        info!("Created new modpack (name = \"{}\") with uuid = {}", &pack.name, pack.id.as_ref().unwrap());
        let id = pack.id.clone().unwrap();
        let clone = pack.clone();
        self.packs.insert(id, pack);
        Ok(clone)
    }

    // Updates the internal settings
    pub fn set_settings(&mut self, settings: settings::Settings) {
        self.settings = settings;
        self.root_folder = Path::new(&self.settings.minecraft.saveDirectory).to_path_buf()
    }

    pub fn launch_modpack(&mut self, id: &str) -> Result<std::process::Child, String> {
        debug!("attempting to launch {}", id);
        match self.get_modpack_mut(id) {
            Some(modpack) => {
                /*
                1. Install launcher to $saveDir/Launcher/
                */
                modpack.lastPlayed = Some(util::get_iso8601(None));
                modpack.timesPlayed = match modpack.timesPlayed {
                    Some(n) => Some(n + 1),
                    None => Some(1)
                };
                let modpack = self.get_modpack(id).unwrap();
                self.save(modpack);
                self.set_launcher_config(&modpack);
                self.run_minecraft_launcher()
            },
            None => Err("No modpack found with id".to_string())
        }
    }

    pub fn get_launcher_exec() -> String {
        match std::env::consts::OS {
            "windows" => "MinecraftLauncher.exe".to_string(),
            "linux" => "minecraft-launcher".to_string(),
            _ => panic!("Unsupported OS")
        }
    }

    pub fn run_minecraft_launcher(&self) -> Result<std::process::Child, String>  {
        let work_dir = self.get_install_folder();
        let executable = &ModpackManager::get_launcher_exec();

        debug!("set work_dir: {:?}", &work_dir);

        std::process::Command::new(self.get_install_folder().join(executable))
            .arg("-w")
            .arg(work_dir)
            .arg("--disableGPU")
            // .arg(self.get_install_folder())
            // .stdout(std::process::Stdio::null())
            // .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())
    }

    #[allow(non_snake_case)]
    fn get_launcher_profile(&self, modpack: &Modpack) -> LauncherProfile {
        let game_dir = self.get_instances_folder().join(&modpack.folder_name.as_ref().unwrap()).to_str().unwrap().to_string();
        let java_args: &str = modpack.settings.javaArgs.as_deref()
            .or(self.settings.minecraft.javaArgs.as_deref())
            .unwrap_or("");
        let memoryMb = match modpack.settings.useCustomMemory {
            true => modpack.settings.javaMemoryMb,
            false => self.settings.minecraft.javaMemoryMb
        };
        let lastVersionId = match modpack.settings.modloaderType.as_str() {
            "forge" => format!("{}-forge-{}", 
                modpack.versions.minecraft, 
                modpack.versions.modloader
            ),
            // "fabric" => format!(""),
            _ => panic!("Unsupported modloader")
        };
        LauncherProfile {
            created: Some(modpack.created.clone()),
            gameDir: Some(game_dir),
            javaArgs: Some(format!("-Xmx{}M {}", memoryMb, java_args)),
            lastUsed: Some(util::get_iso8601(None)),
            icon: Some("Furnace".to_string()),
            lastVersionId,
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
        profiles.insert("CraftyMc".to_string(), self.get_launcher_profile(modpack));
        fs::write(&path, serde_json::to_string_pretty(&profile_config).unwrap()).unwrap();
    }

    pub fn add_mod_entry(&mut self, pack_id: &str, entry: mods::SavedModEntry) -> Modpack {
        let mut pack = self.get_modpack(pack_id).expect("add mod entry to not a modpack").clone();
        let download_dir = self.get_downloads_folder();
        let dest = self.get_instances_folder().join(pack.folder_name.as_ref().unwrap()).join("mods");
        std::fs::create_dir_all(&dest).unwrap();
        let src_path = download_dir.join(&entry.filename);
        let dest_path = dest.join(&entry.filename);
        std::fs::rename(src_path, dest_path).expect("failed to move download");
        pack.mods.push(entry);
        self.save(&pack);
        self.packs.insert(pack.id.clone().unwrap(), pack.clone());
        pack
    }

    pub async fn export(&self, export_type: &str, pack_id: &str, version: String, file_name: &str, paths: &[&str]) -> Result<(), String> {
        let modpack = self.get_modpack(pack_id).expect("unknown modpack");
        let exp_path = self.root_folder.join("Exports").join(file_name);
        let src_path = self.get_instances_folder().join(&modpack.folder_name.as_ref().unwrap());

        match export_type {
            "modrinth" => self.modrinth_manager.as_ref().unwrap().export(version, paths, &modpack, &src_path, exp_path).await,
            _ => self.export_custom(version, paths, &modpack, &src_path, exp_path)
        }.map_err(|err| err.to_string())
    }

    fn export_custom(&self, version: String, paths: &[&str], modpack: &Modpack, src_path: &Path, mut exp_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        exp_path.set_extension("zip");
        let out_file = std::fs::File::create(&exp_path)?;
        let mut zip = zip::ZipWriter::new(out_file);
        let mut buffer = Vec::new();
        // TODO: Modify modpack version
        for path in paths {
            let mut rel_path = path.to_string();
            rel_path.remove(0);
            let file_path = src_path.join(&rel_path);
            if file_path.is_file() {
                let filename = file_path.file_name().ok_or_else(|| "invalid file")?.to_str().ok_or_else(|| "filename str does not exist")?;
                if filename.contains("manifest.json") {
                    continue;
                }
                self.window.as_ref().unwrap().emit("export_progress", payloads::ExportPayload(rel_path.clone()))?;
                match std::fs::File::open(&file_path) {
                    Ok(mut src_file) => {
                        buffer.clear();
                        src_file.read_to_end(&mut buffer)?;
                        zip.start_file(
                            rel_path, 
                            zip::write::FileOptions::default()
                        )?;
                        zip.write_all(&buffer)?;
                    },
                    Err(err) => {
                        warn!("Could not read file \"{}\": {}", &rel_path, err);
                    }
                }
            }
        }
        // Write the manifest with modified pack version
        zip.start_file("manifest.json", zip::write::FileOptions::default())?;
        let mut val = serde_json ::to_value(modpack)?;
        val["versions"]["pack"] = serde_json::Value::String(version);
        zip.write_all(serde_json::to_string_pretty(&val)?.as_bytes())?;

        zip.finish().expect("failed to create zip file");
        util::open_folder(&exp_path)?;
        Ok(())
    }

    pub async fn import(&mut self, path: &PathBuf) -> Result<Modpack, String> {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let instances_dir = self.get_instances_folder();
        std::fs::create_dir_all(&instances_dir).unwrap();
        let import_name = self.get_suitable_name(&filename[0..filename.len() - 7])
            .expect("Could not find available name");
        let window = self.window.as_ref().cloned().unwrap();

        let dest_dir = instances_dir.join(&import_name);
        let zip_file = fs::File::open(path).unwrap();
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();
        info!("Importing {} -> {:?}", &filename, &dest_dir);

        window.emit("update-modpack", payloads::UpdateModpackPayload {
            modpack: None,
            data: None,
            state: payloads::UpdateModpackState::Importing(import_name.to_string(), "Starting import".into())
        }).unwrap();

        match zip.extract(&dest_dir) {
            Ok(()) => {
                if filename.ends_with("mrpack") {
                    debug!("is a modrinth modpack, setting up");
                    if let Err(err) = self.modrinth_manager.as_ref().unwrap().import(&dest_dir).await {
                        return Err(format!("Failed to import modpack: {}", err));
                    }
                }
                debug!("loading entry from disk");
                match self.load_entry(&dest_dir) {
                    Ok(mut pack) => {
                        let id = Uuid::new_v4().to_string();
                        pack.id = Some(id.clone());
                        pack.name = import_name;
                        self.save(&pack);
                        window.emit("update-modpack", payloads::UpdateModpackPayload {
                            modpack: None,
                            data: None,
                            state: payloads::UpdateModpackState::Importing(pack.name.to_string(), "Downloading modloader".into())
                        }).unwrap();
                        match pack.settings.modloaderType.as_str() {
                            "forge" => {
                                debug!("downloading forge {}-{} -- direct", &pack.versions.minecraft, &pack.versions.modloader);
                                match crate::setup::Setup::download_fml_direct(&dest_dir, &pack.versions.minecraft, &pack.versions.modloader).await
                                {
                                    Ok(file) => {
                                        debug!("installing: {}", &file);
                                        pack.versions.modloader = file;
                                        window.emit("update-modpack", payloads::UpdateModpackPayload {
                                            modpack: None,
                                            data: None,
                                            state: payloads::UpdateModpackState::Importing(pack.name.to_string(), "Installing modloader".into())
                                        }).unwrap();
                                        if let Err(err) = self.setup.as_ref().unwrap().install_fml(&mut pack).await { 
                                            return Err(err)
                                        }
                                    },
                                    Err(err) => return Err(err)
                                }
                            },
                            _ => warn!("Unknown modloader \"{}\" for modpack, not installing modloader", &pack.settings.modloaderType )
                        }
                        self.packs.insert(id, pack.clone());
                        return Ok(pack)
                    },
                    Err(e) => return Err(e.to_string())
                }
            },
            Err(err) => Err(err.to_string())
        }
    }    

}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct LauncherProfile {
    pub created: Option<String>,
    pub gameDir: Option<String>,
    pub javaArgs: Option<String>,
    pub lastUsed: Option<String>,
    pub lastVersionId: String,
    pub name: String,
    pub icon: Option<String>,
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
            enableAdvanced: true,
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
