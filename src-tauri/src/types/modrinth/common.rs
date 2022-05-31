
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ModrinthHashes {
    pub sha512: String,
    pub sha1: String,
}