#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::{Arc};
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_log::{LogTarget, LoggerBuilder};
use log::{info, debug, error};
use tokio::sync::MutexGuard;

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
  state.config.blocking_lock().Settings.clone()
}

#[tauri::command]
fn set_setting(state: tauri::State<'_, AppState>, category: &str, key: &str, value: String) -> Result<(), String> {
  // Categories are passed in lowercase. 
  // Keys are case-sensitive, may not be lowercase.
  // TODO: Move to Settings
  let config = &mut state.config.blocking_lock();
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
            let mut setup = setup::Setup::new(&state.modpacks.blocking_lock());
            setup.download_launcher().unwrap();
            state.modpacks.blocking_lock().run_minecraft_launcher().unwrap();
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
  let config = &mut state.config.blocking_lock();
  state.modpacks.blocking_lock().set_settings(config.Settings.clone());
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
  state.modpacks.blocking_lock().create_modpack(modpack)
}

#[tauri::command]
fn get_modpack(state: tauri::State<'_, AppState>, id: &str) -> Option<pack::Modpack> {
  state.modpacks.blocking_lock().get_modpack(id).cloned()
}

#[tauri::command]
fn get_modpacks(state: tauri::State<'_, AppState>) -> Vec<pack::Modpack> {
  state.modpacks.blocking_lock().get_modpacks()
}

#[derive(Clone, serde::Serialize)]
struct UpdateModpackPayload {
  modpack: pack::Modpack,
  deleted: bool
}

#[tauri::command]
// TODO: Possibly not make ui wait for return and instead use events
async fn launch_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, id: &str) -> Result<i32, String> {
  let mut packs = state.modpacks.lock().await;
  match packs.launch_modpack(id) {
    Ok(mut child) => {
      window.emit("update-modpack", UpdateModpackPayload { 
        modpack: packs.get_modpack(id).unwrap().clone(),
        deleted: false
      }).unwrap();
      child.wait().expect("wait for child failed").code().ok_or("killed by signal".to_string())
    },
    Err(err) => Err(err)
  }
}

#[tauri::command]
fn save_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), String> {
  let modpacks = &mut state.modpacks.blocking_lock();
  info!("Saved modpack \"{}\" data", pack_id);
  match modpacks.get_modpack(pack_id) {
    Some(pack) => {
      window.emit("update-modpack", UpdateModpackPayload { 
        modpack: pack.clone(),
        deleted: false
      }).unwrap();
      Ok(modpacks.save(pack))
    },
    None => Err("No modpack was found".to_string())
  }
}

#[tauri::command]
fn set_modpack_setting(state: tauri::State<'_, AppState>, pack_id: &str, key: &str, value: String) -> Result<(), String> {
  let modpacks = &mut state.modpacks.blocking_lock();
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
fn delete_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: String) {
  if let Some(pack) = state.modpacks.blocking_lock().delete_modpack(&pack_id) {
    window.emit("update-modpack", UpdateModpackPayload { 
      modpack: pack,
      deleted: false
    }).unwrap();
  }
}

#[tauri::command]
fn open_modpack_folder(state: tauri::State<'_, AppState>, pack_id: &str) -> Result<(), String> {
  state.modpacks.blocking_lock().open_modpack_folder(pack_id)
}


#[allow(non_snake_case)]
#[tauri::command]
// This works. But I barely understand it. I'm not touching it.
async fn install_mod(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str, author_name: String, mut version_data: mods::ModrinthVersionData) -> Result<(), ()> {
  let mut tuple = fuck_rust(state.modpacks.lock().await, pack_id);
  let entry_data = version_data.install_mod(&window, author_name, &tuple.1, &mut tuple.0).await.unwrap();
  let mut packs = state.modpacks.lock().await;
  let pack = packs.add_mod_entry(pack_id, entry_data);
  window.emit("update-modpack", UpdateModpackPayload { 
    modpack: pack,
    deleted: false
   }).unwrap();
  Ok(())
}


fn fuck_rust(modpacks: MutexGuard<pack::ModpackManager>, pack_id: &str) -> (pack::Modpack, std::path::PathBuf) {
  let pack = modpacks.get_modpack(pack_id).expect("pack not found to install mod to").clone();
  let dest = modpacks.get_downloads_folder();
  (pack, dest)
}

/*
watch_modloader_download -- download has started
  modloader_download_found -- jar found, prompt ui to close
  modloader_download_ready -- window has closed
  modloader_download_complete -- modloader acquired and moved
Ok(())
*/
#[tauri::command]
async fn watch_modloader_download(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), String>{
  match setup::Setup::watch_for_download().await {
    Ok(file) => {
      let modpacks = state.modpacks.lock().await;
      let setup = setup::Setup::new(&modpacks);
      let instances_dir = modpacks.get_instances_folder();
      let modpack = modpacks.get_modpack(pack_id).unwrap().clone();
      let dest_dir = instances_dir.join(&modpack.folder_name.as_ref().unwrap());
      drop(modpacks);

      window.emit("modloader_download_found", EmptyPayload()).unwrap();
      // Wait until window is closed:
      let cl_window = window.clone();
      let cl_modpacks = state.modpacks.clone();
      let pack_id = pack_id.to_string();
      window.once("modloader_download_ready", move | _event | { 
        // Rust can't copy but this can as admin.... stupid but it works
        runas::Command::new(r"C:\Windows\System32\cmd.exe")
          .gui(true)
          .arg("/c")
          .arg("copy")
          .arg(file.path().to_str().unwrap())
          .arg(dest_dir.to_str().unwrap())
          .status()
          .unwrap();

        // Also for fucking idk why I can't run two commands at once. WORKS FINE IN CMD.EXE NORMALLY BUT RUST FUCKS IT UP
        runas::Command::new(r"C:\Windows\System32\cmd.exe")
          .gui(true)
          .arg("/c")
          .arg("del")
          .arg(file.path().to_str().unwrap())
          .status()
          .unwrap();
          
        tokio::spawn(async move {

          let mut modpacks = cl_modpacks.lock().await;
          let mut modpack = modpacks.get_modpack_mut(&pack_id).unwrap().clone();
          modpack.versions.modloader = file.file_name().to_str().unwrap().to_string();

          debug!("waiting for fml install");
          match setup.install_fml(&modpack).await {
            Ok(()) => {
              debug!("fml install complete. finishing modloader setup");
              modpack.versions.modloader = file.file_name().to_str().unwrap().replace("-installer", "");
              cl_window.emit("modloader_download_complete", EmptyPayload()).unwrap();
            },
            Err(msg) => cl_window.emit("modloader_download_error", ErrorPayload(msg.clone())).unwrap()
          };
        });
      });
    },
    Err(msg) => {
      window.emit("modloader_download_error", ErrorPayload(msg.clone())).unwrap();
      return Err(msg);
    }
  };
  Ok(())
}

#[tauri::command]
fn choose_modpack_image(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: String) {
  let modpacks = state.modpacks.blocking_lock();
  let cl_modpacks = state.modpacks.clone();
  let pack = modpacks.get_modpack(&pack_id).unwrap();
  let dest_dir = modpacks.get_instances_folder().join(pack.folder_name.as_ref().unwrap());
  debug!("opening image picker for modpack id = {}", pack_id);
  tauri::api::dialog::FileDialogBuilder::new()
    .set_title("Choose an image for the modpack")
    .add_filter("image", &["png","jpg","webp"])
    .pick_file(move |result| {
      if let Some(filepath) = result {
        let ext = filepath.extension().expect("no file ext").to_str().unwrap();
        std::fs::copy(&filepath, dest_dir.join(
          format!("pack.{}", &ext)
        ));
        let mut modpacks = cl_modpacks.blocking_lock();
        let mut modpack = modpacks.get_modpack_mut(&pack_id).unwrap();
        modpack.img_ext = Some(ext.to_string());
        window.emit("update-modpack", UpdateModpackPayload { 
          modpack: modpack.clone(),
          deleted: false
        }).unwrap();
      }
    });
}


////////////////////////////////////////////////////////////////////////
/// debug Commands
////////////////////////////////////////////////////////////////////////

#[tauri::command]
fn debug_install_launcher(state: tauri::State<'_, AppState>) {
  let mut setup = setup::Setup::new(&state.modpacks.blocking_lock());
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
      create_modpack, get_modpack, get_modpacks, launch_modpack, save_modpack, set_modpack_setting, delete_modpack, watch_modloader_download, open_modpack_folder, choose_modpack_image,
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