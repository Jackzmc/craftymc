#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

mod Settings;

struct AppState {
  settings: Mutex<Settings::Settings>,
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> Settings::Settings {
  state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_setting(state: tauri::State<'_, AppState>, category: &str, key: &str, value: String) -> Option<String> {
  // Categories are passed in lowercase. 
  // Keys are case-sensitive, may not be lowercase.
  // TODO: Move to Settings
  let mut settings = state.settings.lock().unwrap();
  match category {
    "general" => {
      match key {
        _ => return Some("Invalid key".to_string())
      };
      None
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
      None
    }
    _ => return Some("Invalid category".to_string())
  }
  // TODO: Write settings to file
}

fn main() {
  let mut settings = Settings::Settings::new();
  tauri::Builder::default()
    .manage(AppState {
      settings: Mutex::new(settings)
    })
    .invoke_handler(tauri::generate_handler![get_settings, update_setting])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
