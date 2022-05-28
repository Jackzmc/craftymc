use crate::pack;

#[derive(Clone, serde::Serialize)]
#[allow(dead_code)] // False detection
pub enum UpdateModpackState {
  Normal,
  Deleted,
  NowActive,
  Invalid
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