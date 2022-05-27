use crate::AppState;
use crate::setup;
use crate::settings;
use crate::telemetry;
#[allow(unused_imports)]
use log::{info, debug, error, warn};

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> settings::Settings {
  state.config.blocking_lock().settings.clone()
}

#[tauri::command]
pub fn set_setting(state: tauri::State<'_, AppState>, category: &str, key: &str, value: String) -> Result<(), String> {
  // Categories are passed in lowercase. 
  // Keys are case-sensitive, may not be lowercase.
  // TODO: Move to Settings
  let config = &mut state.config.blocking_lock();
  let settings = &mut config.settings;
  debug!("Setting {}/{} to \"{}\"", category, key, &value);
  match category {
    "general" => {
      match key {
        "telemetryState" => {
          let prev = settings.general.telemetryState.clone(); 
          settings.general.telemetryState = value.parse::<i8>().unwrap();
          if prev == -1 && settings.general.telemetryState != prev {
            // First time setup runs here:
            let _ = telemetry::send_telemetry(telemetry::TelemetryFlags::GeneralInfo, &settings);
            let mut setup = setup::Setup::new(&state.modpacks.blocking_lock());
            setup.download_launcher().expect("installing mc launcher failed");
            state.modpacks.blocking_lock().run_minecraft_launcher().expect("running mc launcher failed");
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
pub fn save_settings(state: tauri::State<'_, AppState>) {
  let config = &mut state.config.blocking_lock();
  state.modpacks.blocking_lock().set_settings(config.settings.clone());
  match config.save() {
    Ok(_) => {
      info!("[debug] Saved current settings to file.");
    },
    Err(err) => {
      error!("Failed to save settings to file: {}", err);
    }
  }
  
}