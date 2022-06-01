use crate::pack;

#[derive(Clone, serde::Serialize)]
#[allow(dead_code)] // False detection
pub enum UpdateModpackState {
  Normal,
  Deleted,
  NowActive,
  Invalid(String, String), 
  Importing(String, String)
}

#[derive(Clone, serde::Serialize)]
pub struct UpdateModpackPayload {
  pub modpack: Option<pack::Modpack>,
  pub state: UpdateModpackState,
  pub data: Option<String>
}

#[derive(Clone, serde::Serialize)]
pub struct LauncherStatePayload {
  pub modpack: pack::Modpack,
  pub active: bool
}


#[derive(Clone, serde::Serialize)]
pub struct EmptyPayload();
#[derive(Clone, serde::Serialize)]
pub struct ErrorPayload(pub String);
#[derive(Clone, serde::Serialize)]
pub struct ExportPayload(pub String);

#[derive(Clone, serde::Serialize)]
pub struct ModDownloadedPayload {
    pub mod_id: String,
    pub pack_id: String
}

#[derive(Clone, serde::Serialize)]
pub struct ModDownloadErrorPayload {
    pub mod_id: String,
    pub pack_id: String,
    pub file_name: String,
    pub error: String,
}

use tauri::async_runtime::Mutex;
#[derive(Clone, serde::Serialize)]
pub struct OptionalModRequestPayload(pub Vec<crate::types::modrinth::modpacks::ModrinthModpackFileEntry>);
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionalModResponsePayload(pub Vec<crate::types::modrinth::modpacks::ModrinthModpackFileEntry>);