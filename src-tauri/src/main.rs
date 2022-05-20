#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::{Arc, Mutex};

mod settings;
mod pack;

struct AppState {
  config: Mutex<settings::SettingsManager>,
  modpacks: Mutex<pack::ModpackManager>
}

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
        _ => return Some("Invalid key".to_string())
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

/* TODO: methods:
  save_modpack(name)
  get_modpack(name)
  get_modpacks()
  set_modpack_setting(category, key, value)
  possibly: reload_modpacks
*/

fn main() {
  let config = settings::SettingsManager::new();
  tauri::Builder::default()
    .manage(AppState {
      modpacks: Mutex::new(pack::ModpackManager::new(config.Settings.clone())),
      config: Mutex::new(config)
    })
    .invoke_handler(tauri::generate_handler![
      get_settings, set_setting, save_settings,
      create_modpack, get_modpack, get_modpacks
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
