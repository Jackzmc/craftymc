use crate::AppState;
use crate::payloads;
use crate::types::modrinth;

#[tauri::command]
// This works. But I barely understand it. I'm not touching it.
pub async fn install_mod(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str, author_name: String, mut version_data: modrinth::mods::ModrinthVersionData) -> Result<(), ()> {
  let cl_modpacks = state.modpacks.clone();
  
  let modpacks = state.modpacks.lock().await;
  let mut pack = modpacks.get_modpack(pack_id).expect("pack not found to install mod to").clone();
  let dest = modpacks.get_downloads_folder();
  let entry_data = version_data.install_mod(&window, author_name, &dest, &mut pack).await.unwrap();
  drop(modpacks);

  let mut packs = cl_modpacks.lock().await;
  let pack = packs.add_mod_entry(pack_id, entry_data);
  window.emit("update-modpack", payloads::UpdateModpackPayload { 
    modpack: Some(pack),
    state: payloads::UpdateModpackState::Normal,
    data: None
   }).unwrap();
  Ok(())
}