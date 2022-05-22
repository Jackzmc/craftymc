#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_log::{LogTarget, LoggerBuilder};
use log::{info, debug, error};

mod settings;
mod setup;
mod pack;
mod util;
mod mods;
mod telemetry;

struct AppState {
  config: Mutex<settings::SettingsManager>,
  modpacks: Arc<Mutex<pack::ModpackManager>>
}

////////////////////////////////////////////////////////////////////////
/// Settings Commands
////////////////////////////////////////////////////////////////////////

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> settings::Settings {
  state.config.lock().unwrap().Settings.clone()
}

#[tauri::command]
fn set_setting(state: tauri::State<'_, AppState>, category: &str, key: &str, value: String) -> Result<(), String> {
  // Categories are passed in lowercase. 
  // Keys are case-sensitive, may not be lowercase.
  // TODO: Move to Settings
  let config = &mut state.config.lock().unwrap();
  let settings = &mut config.Settings;
  debug!("Setting {}/{} to \"{}\"", category, key, &value);
  match category {
    "general" => {
      match key {
        "telemetryState" => {
          let prev = settings.general.telemetryState.clone(); 
          settings.general.telemetryState = value.parse::<i8>().unwrap();
          if prev == -1 && settings.general.telemetryState != prev {
            // First time setup runs here:
            let _ = telemetry::send_telemetry(telemetry::TelemetryFlags::GeneralInfo);
            let mut setup = setup::FirstTimeSetup::new(&state.modpacks.lock().unwrap());
            setup.download_launcher();
          }
        },
        _ => return Err("Invalid key".to_string())
      };
    },
    "minecraft" => {
      match key {
        "saveDirectory" => settings.minecraft.saveDirectory = value,
        "preferredRelease" => settings.minecraft.preferredRelease = value,
        "width" => settings.minecraft.width = value.parse::<u32>().unwrap(),
        "height" => settings.minecraft.height = value.parse::<u32>().unwrap(),
        "javaMemoryMb" => settings.minecraft.javaMemoryMb = value.parse::<u32>().unwrap(),
        "javaPath" => settings.minecraft.javaPath = Some(value),
        "javaArgs" => settings.minecraft.javaArgs = Some(value),
        _ => return Err("Invalid, unknown, or unsupported key. Report this to a developer.".to_string())
      };
    }
    _ => return Err("Invalid category".to_string())
  };
  Ok(())
}

#[tauri::command]
fn save_settings(state: tauri::State<'_, AppState>) {
  let config = &mut state.config.lock().unwrap();
  state.modpacks.lock().unwrap().set_settings(config.Settings.clone());
  match config.save() {
    Ok(_) => {
      info!("[debug] Saved current settings to file.");
    },
    Err(err) => {
      error!("Failed to save settings to file: {}", err);
    }
  }
  
}

////////////////////////////////////////////////////////////////////////
/// Modpacks Commands
////////////////////////////////////////////////////////////////////////

#[tauri::command]
fn create_modpack(state: tauri::State<'_, AppState>, modpack: pack::Modpack) -> Result<pack::Modpack, String> {
  state.modpacks.lock().unwrap().create_modpack(modpack)
}

#[tauri::command]
fn get_modpack(state: tauri::State<'_, AppState>, id: &str) -> Option<pack::Modpack> {
  state.modpacks.lock().unwrap().get_modpack(id).cloned()
}

#[tauri::command]
fn get_modpacks(state: tauri::State<'_, AppState>) -> Vec<pack::Modpack> {
  state.modpacks.lock().unwrap().get_modpacks()
}

#[derive(Clone, serde::Serialize)]
struct UpdateModpackPayload {
  modpack: pack::Modpack
}

#[tauri::command]
// TODO: Possibly not make ui wait for return and instead use events
async fn launch_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, id: &str) -> Result<i32, String> {
  let mut packs = state.modpacks.lock().unwrap();
  match packs.launch_modpack(id) {
    Ok(mut child) => {
      window.emit("update-modpack", UpdateModpackPayload { modpack: packs.get_modpack(id).unwrap().clone() }).unwrap();
      child.wait().expect("wait for child failed").code().ok_or("killed by signal".to_string())
    },
    Err(err) => Err(err)
  }
}

#[tauri::command]
fn save_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), String> {
  let modpacks = &mut state.modpacks.lock().unwrap();
  info!("Saved modpack \"{}\" data", pack_id);
  match modpacks.get_modpack(pack_id) {
    Some(pack) => {
      window.emit("update-modpack", UpdateModpackPayload { modpack: pack.clone() }).unwrap();
      Ok(modpacks.save(pack))
    },
    None => Err("No modpack was found".to_string())
  }
}

#[tauri::command]
fn set_modpack_setting(state: tauri::State<'_, AppState>, pack_id: &str, key: &str, value: String) -> Result<(), String> {
  let modpacks = &mut state.modpacks.lock().unwrap();
  let modpack = modpacks.get_modpack_mut(pack_id).expect("[removeme] pack not found");
  debug!("Setting modpack \"{}\" key {} to \"{}\"", pack_id, key, &value);
  match key {
    "name" => modpack.name = value,
    "modloaderType" => modpack.settings.modloaderType = value,
    "minecraft" => modpack.versions.minecraft = value,
    "modloader" => modpack.versions.modloader = value,
    "pack" => modpack.versions.pack = Some(value),
    "javaMemoryMb" => modpack.settings.javaMemoryMb = value.parse::<u32>().unwrap(),
    "javaArgs" => modpack.settings.javaArgs = Some(value),
    "useCustomMemory" => modpack.settings.useCustomMemory = value.parse::<bool>().unwrap(),
    "modSource" => modpack.settings.modSource = value,
    _ => return Err("Unknown key".to_string())
  };
  Ok(())
}

#[tauri::command]
fn delete_modpack(state: tauri::State<'_, AppState>, id: &str) -> Option<pack::Modpack> {
  state.modpacks.lock().unwrap().delete_modpack(id)
}


#[allow(non_snake_case)]
#[tauri::command]
// This works. But I barely understand it. I'm not touching it.
async fn install_mod(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str, author_name: String, mut version_data: mods::ModrinthVersionData) -> Result<(), ()> {
  let mut tuple = fuck_rust(state.modpacks.lock().unwrap(), pack_id);
  let entry_data = version_data.install_mod(&window, author_name, &tuple.1, &mut tuple.0).await.unwrap();
  let mut packs = state.modpacks.lock().unwrap();
  let pack = packs.add_mod_entry(pack_id, entry_data);
  window.emit("update-modpack", UpdateModpackPayload { modpack: pack }).unwrap();
  Ok(())
}


fn fuck_rust(modpacks: std::sync::MutexGuard<pack::ModpackManager>, pack_id: &str) -> (pack::Modpack, std::path::PathBuf) {
  let pack = modpacks.get_modpack(pack_id).expect("pack not found to install mod to").clone();
  let dest = modpacks.get_downloads_folder();
  (pack, dest)
}


#[tauri::command]
async fn watch_modloader_download(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), ()>{
  match watch_for_download() {
    Ok(file) => {
      let modpacks = state.modpacks.lock().unwrap();
      let modpack = modpacks.get_modpack(pack_id).unwrap();
      let dest_dir = modpacks.get_instances_folder().join(&modpack.folder_name.as_ref().unwrap());
      // debug!("found downloaded modloader: {}", &file);
      std::fs::rename(file.path(), dest_dir).expect("mv modloader failed");
      window.emit("modloader_download_complete", EmptyPayload()).unwrap()
    },
    Err(err) => window.emit("modloader_download_error", ErrorPayload(err)).unwrap()
  };
  Ok(())
}


////////////////////////////////////////////////////////////////////////
/// debug Commands
////////////////////////////////////////////////////////////////////////

#[tauri::command]
fn debug_install_launcher(state: tauri::State<'_, AppState>) {
  let mut setup = setup::FirstTimeSetup::new(&state.modpacks.lock().unwrap());
  setup.download_launcher().expect("download launcher failed");
}
fn main() {
  let config = settings::SettingsManager::new();
  let save_folder = std::path::Path::new(&config.Settings.minecraft.saveDirectory).to_path_buf();
  let logs = save_folder.join("Logs");
  tauri::Builder::default()
    .manage(AppState {
      modpacks: Arc::new(Mutex::new(pack::ModpackManager::new(config.Settings.clone()))),
      config: Mutex::new(config),
    })
    .invoke_handler(tauri::generate_handler![
      get_settings, set_setting, save_settings,
      create_modpack, get_modpack, get_modpacks, launch_modpack, save_modpack, set_modpack_setting, delete_modpack, watch_modloader_download,
      install_mod,
      debug_install_launcher
    ])
    .plugin(
      LoggerBuilder::new().targets([
        LogTarget::Folder(logs),
        LogTarget::Stdout,
      ])
      .level(log::LevelFilter::Debug)
      .build()
    ).setup(move |app| {
      let window = app.get_window("main").unwrap();
      window.open_devtools();
      // Move all resources to folder:
      let paths = std::fs::read_dir(app.path_resolver().resource_dir().unwrap()
        .join("_up_").join("resources")
      ).unwrap();
      /*let dest_dir = save_folder.join("Launcher");
      for entry in paths {
        let file = entry.unwrap();
        let path = file.path();
        // TODO: Figure out solution to copy resources to folder.
        debug!("mv {:?} to {:?}", &path, &dest_dir);
        std::fs::copy(&path, &dest_dir).expect("copy failed");
        std::fs::remove_file(&path).expect("rm failed");
      }*/
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

/*
1. UI will emit this, and 5s later open a download window for forge
2. Keep polling until a downloaded jar appears (and optionally check if it looks valid, like forge-xxx)
3. Send event to UI telling them you got it:
  a. ui will close window handle and show mod list
  b. rust will attempt to run the installer in the background?
*/
#[derive(Clone, serde::Serialize)]
struct EmptyPayload();
#[derive(Clone, serde::Serialize)]
struct ErrorPayload(String);

fn watch_for_download() -> Result<std::fs::DirEntry, String> {
  let downloads_dir = &dirs_next::download_dir().expect("cannot find download dir");
  let now = std::time::SystemTime::now();
  while now.elapsed().unwrap().as_secs() < 120 {
    let paths = std::fs::read_dir(downloads_dir).expect("cannot read dir");
    for path in paths {
      let file = path.unwrap();
      match file.metadata().unwrap().created() { 
        Ok(created) => {
          if file.file_type().unwrap().is_file() && created.duration_since(now).unwrap().as_secs() <= 60 {
            let filename = &file.file_name().into_string().unwrap();
            if filename.ends_with(".jar") {
              return Ok(file)
            }
          }
        },
        Err(err) => return Err(err.to_string())
      };
    }
  }
  Err("Watch timed out".to_string())
}