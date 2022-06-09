use std::io::Write;
use futures::{StreamExt};
use log::{info, debug, error};
use crate::pack;
use crate::payloads;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SavedModEntry {
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    pub filename: String,
    pub name: Option<String>,
    pub author: Option<String>,
    pub sha512: Option<String>,
    pub sha1: Option<String>,
}

impl crate::types::modrinth::mods::ModrinthVersionData {
    pub async fn install_mod(&mut self, window: &tauri::Window, 
        author_name: String, destination: &std::path::PathBuf, pack: &mut pack::Modpack
    ) -> Result<Vec<SavedModEntry>, String> {
        let client = reqwest::Client::new();
        let pack_id = pack.id.as_deref().unwrap();
        let mut filenames = Vec::<String>::new();
        info!("Downloading {} files for pack id {}", self.files.len(), pack_id);
        for file in &self.files {
            debug!("[debug] Downloading file = {}, bytes = {}", &file.filename, &file.size);
            let mut dest = std::fs::File::create(destination.join(&file.filename)).expect("Could not create file");
            match client
                .get(&file.url)
                .header("User-Agent", "mc-mod-manager/v1.0-alpha")
                .send()
                .await
            {
                Ok(response) => {
                    let mut stream = response.bytes_stream();
                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(chunk) => {
                                if let Err(err) = dest.write(&chunk) {
                                    println!("[{}] Write Error: {}", &file.filename, err);
                                    break;
                                }
                            },
                            Err(err) => {
                                window.emit("download-mod", payloads::ModDownloadErrorPayload {
                                    mod_id: self.id.clone(),
                                    file_name: file.filename.clone(),
                                    pack_id: pack_id.to_string(),
                                    error: err.to_string()
                                }).ok();
                                println!("item {} failed:\n{}", &file.filename, &err); 
                                return Err(err.to_string())
                            }
                        }
                    }
                    debug!("{}: finished", &file.filename);
                    window.emit("download-mod", payloads::ModDownloadedPayload {
                        mod_id: self.id.clone(),
                        pack_id: pack_id.to_string()
                    }).ok();
                    filenames.push(file.filename.clone());
                    // TODO: insert into pack.mods
                },
                Err(err) => {
                    error!("Download failure for {}: {}", &file.filename, err);
                    return Err(err.to_string())
                }
            }
        }
        info!("[debug] Completed download queue for {}", pack_id);
        let result = filenames.into_iter()
        .map(|filename| {
            SavedModEntry {
                project_id: Some(self.project_id.clone()),
                version_id: Some(self.id.clone()),
                filename,
                name: Some(self.name.clone()),
                author: Some(author_name.clone()),
                sha512: None,
                sha1: None
            }
        })
        .collect();
        Ok(result)
    }
}

