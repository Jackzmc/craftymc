#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::{Arc, Mutex};
use std::io::prelude::*;

mod settings;

struct AppState {
  config: Mutex<settings::SettingsManager>,
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
  let settings = &config.Settings;
  let json_str = serde_json::to_string(&settings).unwrap();
  match config.config_file.write_all(&json_str.as_bytes()) {
    Ok(_) => {
      config.config_file.flush().unwrap();
      println!("[debug] Saved settings to file.");
    },
    Err(err) => {
      println!("WARN: Failed to save settings to file: {}", err);
    }
  }
  
}

fn main() {
  let config = settings::SettingsManager::new();
  tauri::Builder::default()
    .manage(AppState {
      config: Mutex::new(config)
    })
    .invoke_handler(tauri::generate_handler![get_settings, set_setting, save_settings])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
