use crate::pack;

#[derive(Clone, serde::Serialize)]
pub struct UpdateModpackPayload {
  pub modpack: pack::Modpack,
  pub deleted: bool
}

#[derive(Clone, serde::Serialize)]
pub struct EmptyPayload();
#[derive(Clone, serde::Serialize)]
pub struct ErrorPayload(pub String);
#[derive(Clone, serde::Serialize)]
pub struct ExportPayload(pub String);