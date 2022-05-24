use crate::setup;
use crate::AppState;
use crate::pack;
use crate::payloads;
use tauri::Runtime;
#[allow(unused_imports)]
use log::{info, debug, error, warn};
/// COMMANDS

#[tauri::command]
fn create_modpack(state: tauri::State<'_, AppState>, modpack: pack::Modpack) -> Result<pack::Modpack, String> {
  state.modpacks.blocking_lock().create_modpack(modpack)
}

#[tauri::command]
fn get_modpack(state: tauri::State<'_, AppState>, id: &str) -> Option<pack::Modpack> {
  state.modpacks.blocking_lock().get_modpack(id).cloned()
}

#[tauri::command]
fn get_modpacks(state: tauri::State<'_, AppState>) -> Vec<pack::Modpack> {
  state.modpacks.blocking_lock().get_modpacks()
}
#[tauri::command]
// TODO: Possibly not make ui wait for return and instead use events
async fn launch_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, id: &str) -> Result<i32, String> {
  let mut packs = state.modpacks.lock().await;
  match packs.launch_modpack(id) {
    Ok(mut child) => {
      window.emit("update-modpack", payloads::UpdateModpackPayload { 
        modpack: packs.get_modpack(id).unwrap().clone(),
        deleted: false
      }).unwrap();
      child.wait().expect("wait for child failed").code().ok_or("killed by signal".to_string())
    },
    Err(err) => Err(err)
  }
}

#[tauri::command]
fn save_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), String> {
  let modpacks = &mut state.modpacks.blocking_lock();
  info!("Saved modpack \"{}\" data", pack_id);
  match modpacks.get_modpack(pack_id) {
    Some(pack) => {
      window.emit("update-modpack", payloads::UpdateModpackPayload { 
        modpack: pack.clone(),
        deleted: false
      }).unwrap();
      Ok(modpacks.save(pack))
    },
    None => Err("No modpack was found".to_string())
  }
}

#[tauri::command]
fn set_modpack_setting(state: tauri::State<'_, AppState>, pack_id: &str, key: &str, value: String) -> Result<(), String> {
  let modpacks = &mut state.modpacks.blocking_lock();
  let modpack = modpacks.get_modpack_mut(pack_id).expect("[removeme] pack not found");
  debug!("Setting modpack \"{}\" key {} to \"{}\"", pack_id, key, &value);
  match key {
    "name" => modpack.name = value,
    "modloaderType" => modpack.settings.modloaderType = value,
    "minecraft" => modpack.versions.minecraft = value,
    "modloader" => modpack.versions.modloader = value,
    "pack" => modpack.versions.pack = Some(value),
    "javaMemoryMb" => modpack.settings.javaMemoryMb = value.parse::<u32>().unwrap(),
    "javaArgs" => modpack.settings.javaArgs = Some(value),
    "useCustomMemory" => modpack.settings.useCustomMemory = value.parse::<bool>().unwrap(),
    "modSource" => modpack.settings.modSource = value,
    _ => return Err("Unknown key".to_string())
  };
  Ok(())
}

#[tauri::command]
fn delete_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: String) {
  if let Some(pack) = state.modpacks.blocking_lock().delete_modpack(&pack_id) {
    window.emit("update-modpack", payloads::UpdateModpackPayload { 
      modpack: pack,
      deleted: false
    }).unwrap();
  }
}

#[tauri::command]
fn open_modpack_folder(state: tauri::State<'_, AppState>, pack_id: &str) -> Result<(), String> {
  state.modpacks.blocking_lock().open_modpack_folder(pack_id)
}

/*
watch_modloader_download -- download has started
  modloader_download_found -- jar found, prompt ui to close
  modloader_download_ready -- window has closed
  modloader_download_complete -- modloader acquired and moved
Ok(())
*/
#[tauri::command]
async fn watch_modloader_download(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), String>{
  match setup::Setup::watch_for_download().await {
    Ok(file) => {
      let modpacks = state.modpacks.lock().await;
      let setup = setup::Setup::new(&modpacks);
      let instances_dir = modpacks.get_instances_folder();
      let modpack = modpacks.get_modpack(pack_id).unwrap().clone();
      let dest_dir = instances_dir.join(&modpack.folder_name.as_ref().unwrap());
      drop(modpacks);

      window.emit("modloader_download_found", payloads::EmptyPayload()).unwrap();
      // Wait until window is closed:
      let cl_window = window.clone();
      let cl_modpacks = state.modpacks.clone();
      let pack_id = pack_id.to_string();
      window.once("modloader_download_ready", move | _event | { 
        // Rust can't copy but this can as admin.... stupid but it works
        runas::Command::new(r"C:\Windows\System32\cmd.exe")
          .gui(true)
          .arg("/c")
          .arg("copy")
          .arg(file.path().to_str().unwrap())
          .arg(dest_dir.to_str().unwrap())
          .status()
          .unwrap();

        // Also for fucking idk why I can't run two commands at once. WORKS FINE IN CMD.EXE NORMALLY BUT RUST FUCKS IT UP
        runas::Command::new(r"C:\Windows\System32\cmd.exe")
          .gui(true)
          .arg("/c")
          .arg("del")
          .arg(file.path().to_str().unwrap())
          .status()
          .unwrap();
          
        tokio::spawn(async move {

          let mut modpacks = cl_modpacks.lock().await;
          let mut modpack = modpacks.get_modpack_mut(&pack_id).unwrap().clone();
          modpack.versions.modloader = file.file_name().to_str().unwrap().to_string();

          debug!("waiting for fml install");
          match setup.install_fml(&modpack).await {
            Ok(()) => {
              debug!("fml install complete. finishing modloader setup");
              modpack.versions.modloader = file.file_name().to_str().unwrap().replace("-installer", "");
              cl_window.emit("modloader_download_complete", payloads::EmptyPayload()).unwrap();
            },
            Err(msg) => cl_window.emit("modloader_download_error", payloads::ErrorPayload(msg.clone())).unwrap()
          };
        });
      });
    },
    Err(msg) => {
      window.emit("modloader_download_error", payloads::ErrorPayload(msg.clone())).unwrap();
      return Err(msg);
    }
  };
  Ok(())
}

#[tauri::command]
fn choose_modpack_image(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: String) {
  let modpacks = state.modpacks.blocking_lock();
  let cl_modpacks = state.modpacks.clone();
  let pack = modpacks.get_modpack(&pack_id).unwrap();
  let dest_dir = modpacks.get_instances_folder().join(pack.folder_name.as_ref().unwrap());
  debug!("opening image picker for modpack id = {}", pack_id);
  tauri::api::dialog::FileDialogBuilder::new()
    .set_title("Choose an image for the modpack")
    .add_filter("image", &["png","jpg","webp"])
    .pick_file(move |result| {
      if let Some(filepath) = result {
        let ext = filepath.extension().expect("no file ext").to_str().unwrap();
        std::fs::copy(&filepath, dest_dir.join(
          format!("pack.{}", &ext)
        ));
        let mut modpacks = cl_modpacks.blocking_lock();
        let mut modpack = modpacks.get_modpack_mut(&pack_id).unwrap();
        modpack.img_ext = Some(ext.to_string());
        window.emit("update-modpack", payloads::UpdateModpackPayload { 
          modpack: modpack.clone(),
          deleted: false
        }).unwrap();
      }
    });
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("modpacks")
      .invoke_handler(tauri::generate_handler![
        create_modpack, get_modpack, get_modpacks, launch_modpack, save_modpack, 
        set_modpack_setting, delete_modpack, watch_modloader_download, open_modpack_folder, 
        choose_modpack_image,
      ])
      .build()
}
  
  