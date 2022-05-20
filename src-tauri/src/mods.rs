struct ModManager {
    
}

pub struct DownloadedMod {
    pub name: String,
    pub id: String
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModrinthModData {
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub featured: bool,
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub changelog_url: Option<String>,
    pub date_published: String,
    pub downloads: i64,
    pub version_type: String,
    pub files: Vec<ModrinthFile>,
    // pub dependencies: Vec<Value>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModrinthFile {
    pub hashes: ModrinthHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModrinthHashes {
    pub sha512: String,
    pub sha1: String,
}