use crate::mods;
use crate::pack;
use crate::AppState;
use tokio::sync::MutexGuard;
use crate::payloads;



pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
  tauri::plugin::Builder::new("mods")
    .invoke_handler(tauri::generate_handler![])
    .build()
}

