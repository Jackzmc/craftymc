use crate::setup;
use crate::settings;
use crate::AppState;
use crate::telemetry;
#[allow(unused_imports)]
use log::{info, debug, error, warn};

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

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("config")
        .invoke_handler(tauri::generate_handler![
          get_settings, set_setting, save_settings,
        ])
        .build()
  }
  