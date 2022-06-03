#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::{Arc};
use tauri::async_runtime::Mutex;
use tauri_plugin_log::{LogTarget, LoggerBuilder};
use tauri::Manager;
#[allow(unused_imports)]
use log::{info, debug, error, warn};

mod settings;
mod setup;
mod pack;
mod util;
mod mods;
mod telemetry;
pub mod commands;
pub mod types;
mod payloads;

use commands::{cmd_packs, cmd_mods, cmd_settings, cmd_debug};

pub struct AppState{
  config: Mutex<settings::SettingsManager>,
  modpacks: Arc<Mutex<pack::ModpackManager>>
}

fn main() {
  let config = settings::SettingsManager::new();
  let save_folder = std::path::Path::new(&config.settings.minecraft.saveDirectory).to_path_buf();
  let logs = save_folder.join("Logs");

  tauri::Builder::default()
    .manage(AppState {
      modpacks: Arc::new(Mutex::new(pack::ModpackManager::new(config.settings.clone()))),
      config: Mutex::new(config),
    })
    .invoke_handler(tauri::generate_handler![
      cmd_packs::launch_modpack, cmd_packs::watch_modloader_download, cmd_packs::save_modpack, cmd_packs::delete_modpack, cmd_packs::choose_modpack_image, cmd_packs::create_modpack, cmd_packs::get_modpack, cmd_packs::get_modpacks, cmd_packs::set_modpack_setting, cmd_packs::open_modpack_folder, cmd_packs::export_modpack, cmd_packs::get_instance_tree, cmd_packs::import_modpack, cmd_packs::install_modpack,
      cmd_mods::install_mod,
      cmd_debug::debug_install_launcher,
      cmd_settings::get_settings, cmd_settings::set_setting, cmd_settings::save_settings
    ])
    .plugin(
      LoggerBuilder::new().targets([
        LogTarget::Folder(logs),
        LogTarget::Stdout,
      ])
      .level(log::LevelFilter::Debug)
      .build()
    )
    .setup(move |app| {
      let window = app.get_window("main").unwrap();
      #[cfg(debug_assertions)]
      window.open_devtools();

      let state = app.state::<AppState>();
      let mut packs = state.modpacks.blocking_lock();
      let mut config = state.config.blocking_lock();
      packs.provide_window(window.clone());
      config.set_version(app.config().package.version.as_ref().expect("missing version").clone());
      if config.settings.general.telemetryState == -1 {
        info!("Running first time setup");
        let mut setup = setup::Setup::new(&packs);
        if !packs.get_install_folder().join(pack::ModpackManager::get_launcher_exec()).exists() {
          setup.download_launcher().expect("installing mc launcher failed");
          packs.run_minecraft_launcher().expect("running mc launcher failed");
        } else {
          info!("Skipping download & install of launcher: already exists");
        }
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
