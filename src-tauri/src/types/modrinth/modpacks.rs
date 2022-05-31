#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpackManifest {
    pub format_version: String,
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub files: Vec<ModrinthModpackFileEntry>,
    pub dependencies: ModrinthManifestDependency, 
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpackFileEntry {
    pub path: String,
    pub hashes: crate::types::modrinth::common::ModrinthHashes,
    pub env: Option<ModrinthEnv>,
    pub downloads: Option<String>,
    pub file_size: usize
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ModrinthEnv {
    pub client: String,
    pub server: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ModrinthManifestDependency {
    pub fabric_loader: Option<String>,
    pub minecraft: Option<String>,
    pub forge: Option<String>,
    pub quilt_loader: Option<String> 
}

