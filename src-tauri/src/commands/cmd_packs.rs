use crate::setup;
use crate::AppState;
use crate::pack;
use crate::payloads;
use crate::util;
use crate::types::modrinth;
use futures::stream::StreamExt;
#[allow(unused_imports)]
use log::{info, debug, error, warn};
/// COMMANDS

#[tauri::command]
pub fn create_modpack(state: tauri::State<'_, AppState>, modpack: pack::Modpack) -> Result<pack::Modpack, String> {
  state.modpacks.blocking_lock().create_modpack(modpack)
}

#[tauri::command]
pub fn get_modpack(state: tauri::State<'_, AppState>, id: &str) -> Option<pack::Modpack> {
  state.modpacks.blocking_lock().get_modpack(id).cloned()
}

#[tauri::command]
pub fn get_modpacks(state: tauri::State<'_, AppState>) -> Vec<pack::Modpack> {
  state.modpacks.blocking_lock().get_modpacks()
}



#[tauri::command]
pub fn set_modpack_setting(state: tauri::State<'_, AppState>, pack_id: &str, key: &str, value: String) -> Result<(), String> {
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
pub fn open_modpack_folder(state: tauri::State<'_, AppState>, pack_id: &str) -> Result<(), String> {
  let modpacks = state.modpacks.blocking_lock();
  let modpack = modpacks.get_modpack(pack_id).unwrap();
  let path = modpacks.get_instances_folder().join(&modpack.folder_name.as_ref().unwrap());
  util::open_folder(&path).map(|_| ())
}
  
/*
cmd  start_modloader_download -- rust creates window, with tmp user data it can access
 :requires new window resolving forge/fabric url 
rust watch_for_download
evnt modloader_download_found -- jar found, prompt ui to close
evnt modloader_download_ready -- window has closede
evnt modloader_download_complete -- modloader acquired and moved
Ok(())
*/
#[tauri::command]
pub async fn watch_modloader_download(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), String>{
  match setup::Setup::watch_for_download().await {
    Ok(file) => {
      let modpacks = state.modpacks.lock().await;
      let setup = setup::Setup::new(&modpacks);
      let instances_dir = modpacks.get_instances_folder();
      let modpack = modpacks.get_modpack(pack_id).unwrap().clone();
      let dest_path = instances_dir.join(&modpack.folder_name.as_ref().unwrap());
      drop(modpacks);

      debug!("Found modloader installer: {:?}", file.file_name());
      window.emit("modloader_download_found", payloads::EmptyPayload()).unwrap();
      // Wait until window is closed:
      let cl_window = window.clone();
      let cl_modpacks = state.modpacks.clone();
      let pack_id = pack_id.to_string();
      window.once("modloader_download_ready", move | _event | { 
        // Rust can't copy but this can as admin.... stupid but it works
        // TODO: Linux support
        let src_path = file.path();
        util::mv_as_admin(&src_path, &dest_path);
          
        tokio::spawn(async move {

          let mut modpacks = cl_modpacks.lock().await;
          let mut modpack = modpacks.get_modpack_mut(&pack_id).unwrap().clone();
          // Pass installer name to fml
          modpack.versions.modloader = file.file_name().to_str().unwrap().to_string();

          debug!("waiting for fml install");
          match setup.install_fml(&mut modpack).await {
            Ok(()) => {
              debug!("fml install complete. finishing modloader setup");
              cl_window.emit("modloader_download_complete", payloads::EmptyPayload()).unwrap();
              modpacks.save(&modpack);
              modpacks.replace(modpack);
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
// TODO: Possibly not make ui wait for return and instead use events
pub async fn launch_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, id: &str) -> Result<(), String> {
  let mut packs = state.modpacks.lock().await;
  match packs.launch_modpack(id) {
    Ok(mut child) => {
      window.emit("update-modpack", payloads::UpdateModpackPayload { 
        modpack: Some(packs.get_modpack(id).unwrap().clone()),
        state: payloads::UpdateModpackState::NowActive,
        data: None
      }).unwrap();
      let exit_code = match child.wait().expect("wait for child failed").code() {
        Some(code) => Some(code.to_string()),
        None => None
      };
      window.emit("update-modpack", payloads::UpdateModpackPayload { 
        modpack: None,
        state: payloads::UpdateModpackState::Normal,
        data: exit_code
      }).unwrap();
      Ok(())
    },
    Err(err) => Err(err)
  }
}



#[tauri::command]
pub fn choose_modpack_image(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: String) {
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
        )).expect("copy pack image failed");
        let mut modpacks = cl_modpacks.blocking_lock();
        let mut modpack = modpacks.get_modpack_mut(&pack_id).unwrap();
        modpack.img_ext = Some(ext.to_string());
        window.emit("update-modpack", payloads::UpdateModpackPayload { 
          modpack: Some(modpack.clone()),
          state: payloads::UpdateModpackState::Normal,
          data: None
        }).unwrap();
      }
    });
}

#[tauri::command]
pub fn delete_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: String) -> Result<(), String> {
  match state.modpacks.blocking_lock().delete_modpack(&pack_id) {
    Ok(Some(pack)) => {
      window.emit("update-modpack", payloads::UpdateModpackPayload { 
        modpack: Some(pack),
        state: payloads::UpdateModpackState::Deleted,
        data: None
      }).unwrap();
      Ok(())
    },
    Err(err) => Err(err),
    _ => Ok(())
  }
}


#[tauri::command]
pub fn save_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, pack_id: &str) -> Result<(), String> {
  let modpacks = &mut state.modpacks.blocking_lock();
  info!("Saved modpack \"{}\" data", pack_id);
  match modpacks.get_modpack(pack_id) {
    Some(modpack) => {
      modpacks.save(modpack);
      window.emit("update-modpack", payloads::UpdateModpackPayload { 
        modpack: Some(modpack.clone()),
        state: payloads::UpdateModpackState::Normal,
        data: None
      }).unwrap();
      Ok(())
    },
    None => Err("No modpack was found".to_string())
  }
}

#[tauri::command]
pub async fn export_modpack(state: tauri::State<'_, AppState>,
  pack_id: &str, file_name: &str, version: String, paths: Vec<&str>, export_type: &str
) -> Result<(), String> {
  let modpacks = &mut state.modpacks.lock().await;
  match modpacks.get_modpack(pack_id) {
    Some(pack) => {
      info!("Exporting modpack id = {}", &pack.id.as_ref().unwrap());
      modpacks.export(export_type, pack_id, version, file_name, &paths).await
    },
    None => Err("No modpack was found".to_string())
  }
}

enum ChannelRes {
  Data(std::path::PathBuf),
  Cancel
}
#[tauri::command]
pub async fn import_modpack(state: tauri::State<'_, AppState>, window: tauri::Window) -> Result<(), String> { 
  let modpacks = state.modpacks.clone();
  let (tx, rx) = std::sync::mpsc::channel();
  tauri::api::dialog::FileDialogBuilder::new()
    .set_title("Import a modpack")
    .add_filter("modpack archive", &["zip", "mrpack"])
    .pick_file(move |result| {
      if let Some(filepath) = result {
        tx.send(ChannelRes::Data(filepath)).unwrap();
      } else { 
        tx.send(ChannelRes::Cancel).unwrap();
      }
    });
    
  let res = rx.recv().unwrap();
  drop(rx);
  if let ChannelRes::Data(filepath) = res {
    let mut modpacks = modpacks.lock().await;
    debug!("starting import of {:?}", &filepath);
    match modpacks.import(&filepath).await {
      Ok(modpack) => {
        std::fs::remove_file(filepath).map_err(|x| x.to_string())?;
        window.emit("update-modpack", payloads::UpdateModpackPayload { 
          modpack: Some(modpack),
          state: payloads::UpdateModpackState::Normal,
          data: None
        }).unwrap();
      },
      Err(_) => {
        // TODO: Pass to ui
      }
    }
  };
  Ok(())
}

#[tauri::command]
pub async fn get_instance_tree(state: tauri::State<'_, AppState>, pack_id: &str) -> Result<util::TreeEntry, String> {
  let modpacks = state.modpacks.lock().await;
  let pack = modpacks.get_modpack(pack_id).unwrap();
  let root = modpacks.get_instances_folder().join(pack.folder_name.as_ref().unwrap());
  Ok(util::get_directory_tree(&root))
}

#[tauri::command]
// Possibly move this to a cmd_modrinth
pub async fn install_modpack(state: tauri::State<'_, AppState>, window: tauri::Window, 
  project_id: &str, author_name: &str, version_data: modrinth::mods::ModrinthVersionData
) -> Result<(), String> {
  let mut modpacks = state.modpacks.lock().await;
  let project = modpacks.modrinth_manager.as_ref().unwrap().fetch(project_id).await?;
  let download_file = modpacks.get_downloads_folder().join(format!("{}.mrpack", project.slug));

  debug!("downloading {:?}", &version_data.files[0].url);
  crate::setup::Setup::download_file(&download_file, &version_data.files[0].url).await.map_err(|err| err.to_string())?;
  
  debug!("starting import of {:?}", &download_file);
  match modpacks.import(&download_file).await {
    Ok(mut modpack) => {
      if let Some(url_str) = project.icon_url {
        let url = reqwest::Url::parse(&url_str).unwrap();
        let segments = url.path_segments().unwrap();
        let segments = segments.last().unwrap();
        let ext = segments.split(".").last().unwrap();
        let dest = modpacks.get_instances_folder().join(modpack.folder_name.as_ref().unwrap()).join(format!("pack.{}", &ext));
        if let Err(err) = crate::setup::Setup::download_file(&dest, &url_str).await {
          warn!("Failed to download pack image from {}", err);
        } else {
          modpack.img_ext = Some(ext.to_string());
        }
      }
      // Not efficient because modrinth doesn't include file_name right now:
      if let Some(dependencies) = version_data.dependencies {
        debug!("Fetching dependencies, this may take a while");
        let version_ids = dependencies.into_iter().map(|depend| depend.version_id);
        futures::stream::iter(version_ids)
        .map(|version_id| {
          modpacks.modrinth_manager.as_ref().unwrap().fetch_version(version_id)
        })
        .buffer_unordered(4)
        .for_each(|version| {
          if let Ok(version) = version {
            for entry in &mut modpack.mods {
              if &entry.filename == &version.files[0].filename {
                entry.version_id = Some(version.id);
                // entry.author = verison.author_id
                entry.name = Some(version.name);
                break;
              }
            }
          }
          async {}
        })
        .await;
      }

      debug!("import finished, setting some properties and saving");
      
      modpack.name = modpacks.get_suitable_name(&project.title).unwrap();
      modpack.author = Some(author_name.to_string());

      modpacks.save(&modpack);
      let id = modpack.id.as_ref().unwrap().to_string();
      modpacks.packs.insert(id, modpack.clone());

      window.emit("update-modpack", payloads::UpdateModpackPayload { 
        modpack: Some(modpack),
        state: payloads::UpdateModpackState::Normal,
        data: None
      }).unwrap();
    },
    Err(_) => {
      // TODO: Pass to ui
    }
  }
  Ok(())
}
