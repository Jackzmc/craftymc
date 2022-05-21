#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::{Arc, Mutex};

mod settings;
mod pack;
mod util;
mod mods;

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
fn set_setting(state: tauri::State<'_, AppState>, category: &str, key: &str, value: String) -> Option<String> {
  // Categories are passed in lowercase. 
  // Keys are case-sensitive, may not be lowercase.
  // TODO: Move to Settings
  let config = &mut state.config.lock().unwrap();
  let settings = &mut config.Settings;
  println!("[debug] Setting {}/{} to \"{}\"", &category, &key, &value);
  match category {
    "general" => {
      match key {
        _ => return Some("Invalid key".to_string())
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
        _ => return Some("Invalid, unknown, or unsupported key. Report this to a developer.".to_string())
      };
    }
    _ => return Some("Invalid category".to_string())
  };
  None
}

#[tauri::command]
fn save_settings(state: tauri::State<'_, AppState>) {
  let config = &mut state.config.lock().unwrap();
  state.modpacks.lock().unwrap().set_settings(config.Settings.clone());
  match config.save() {
    Ok(_) => {
      println!("[debug] Saved settings to file.");
    },
    Err(err) => {
      println!("WARN: Failed to save settings to file: {}", err);
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
fn delete_modpack(state: tauri::State<'_, AppState>, id: &str) -> Option<pack::Modpack> {
  state.modpacks.lock().unwrap().delete_modpack(id)
}


#[allow(non_snake_case)]
#[tauri::command]
// TODO: async this shit. grr
async fn install_mod(state: tauri::State<'_, AppState>, pack_id: &str, window: tauri::Window, mut mod_data: mods::ModrinthModData) -> Result<(), ()> {
  let mut tuple = fuck_rust(state.modpacks.lock().unwrap(), pack_id);
  mod_data.install_mod(&tuple.1, &window, &mut tuple.0).await.unwrap();
  Ok(())
}

fn fuck_rust(modpacks: std::sync::MutexGuard<pack::ModpackManager>, pack_id: &str) -> (pack::Modpack, std::path::PathBuf) {
  let mut pack = modpacks.get_modpack(pack_id).expect("pack not found to install mod to").clone();
  let dest = modpacks.get_downloads_folder();
  (pack, dest)
}

////////////////////////////////////////////////////////////////////////
/// Mod Commands
////////////////////////////////////////////////////////////////////////

// Possibly use a queue based downloader... but lazy mode just do it async


/* TODO: methods:
  save_modpack(name)
  [x] get_modpack(name)
  [x] get_modpacks()
  set_modpack_setting(category, key, value)
  possibly: reload_modpacks
*/

fn main() {
  let config = settings::SettingsManager::new();
  tauri::Builder::default()
    .manage(AppState {
      modpacks: Arc::new(Mutex::new(pack::ModpackManager::new(config.Settings.clone()))),
      config: Mutex::new(config),
    })
    .invoke_handler(tauri::generate_handler![
      get_settings, set_setting, save_settings,
      create_modpack, get_modpack, get_modpacks, launch_modpack, delete_modpack,
      install_mod
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
