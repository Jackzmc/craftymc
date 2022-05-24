use crate::mods;
use crate::pack;
use crate::AppState;
use tokio::sync::MutexGuard;
use crate::payloads;

  
#[tauri::command]
// This works. But I barely understand it. I'm not touching it.
async fn install_mod(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str, author_name: String, mut version_data: mods::ModrinthVersionData) -> Result<(), ()> {
  let mut tuple = fuck_rust(state.modpacks.lock().await, pack_id);
  let entry_data = version_data.install_mod(&window, author_name, &tuple.1, &mut tuple.0).await.unwrap();
  let mut packs = state.modpacks.lock().await;
  let pack = packs.add_mod_entry(pack_id, entry_data);
  window.emit("update-modpack", payloads::UpdateModpackPayload { 
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

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
  tauri::plugin::Builder::new("mods")
    .invoke_handler(tauri::generate_handler![install_mod])
    .build()
}

