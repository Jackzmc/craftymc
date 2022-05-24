#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::{Arc};
use tauri::async_runtime::Mutex;
use tauri_plugin_log::{LogTarget, LoggerBuilder};
use tauri::Manager;

mod settings;
mod setup;
mod pack;
mod util;
mod mods;
mod telemetry;
mod commands;
mod payloads;

struct AppState {
  config: Mutex<settings::SettingsManager>,
  modpacks: Arc<Mutex<pack::ModpackManager>>
}

////////////////////////////////////////////////////////////////////////
/// Settings Commands
////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////
/// Modpacks Commands
////////////////////////////////////////////////////////////////////////






fn main() {
  let config = settings::SettingsManager::new();
  let save_folder = std::path::Path::new(&config.Settings.minecraft.saveDirectory).to_path_buf();
  let logs = save_folder.join("Logs");

  tauri::Builder::default()
    .manage(AppState {
      modpacks: Arc::new(Mutex::new(pack::ModpackManager::new(config.Settings.clone()))),
      config: Mutex::new(config),
    })
    .invoke_handler(tauri::generate_handler![])
    .plugin(
      LoggerBuilder::new().targets([
        LogTarget::Folder(logs),
        LogTarget::Stdout,
      ])
      .level(log::LevelFilter::Debug)
      .build()
    )
    .plugin(commands::debug::init())
    .plugin(commands::settings::init())
    .plugin(commands::modpacks::init())
    .plugin(commands::mods::init())
    .setup(move |app| {
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
