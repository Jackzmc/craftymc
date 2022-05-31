#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ModrinthVersionData {
    pub id: String,
    pub project_id: String,
    pub author_id: Option<String>,
    pub team: Option<String>,
    pub featured: bool,
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    pub changelog_url: Option<String>,
    pub date_published: String,
    pub downloads: i64,
    pub version_type: String,
    pub files: Vec<ModrinthFile>,
    pub dependencies: Option<Vec<ModrinthDependency>>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ModrinthFile {
    pub hashes: crate::types::modrinth::common::ModrinthHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ModrinthDependency {
    version_id: String,
    project_id: String,
    dependency_type: String
}
