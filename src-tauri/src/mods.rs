use std::io::Write;
use futures::{StreamExt};
use log::{info, debug, error};
use crate::pack;
use crate::payloads;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SavedModEntry {
    pub project_id: String,
    pub version_id: String,
    pub filenames: Vec<String>,
    pub name: String,
    pub author: String
}

impl crate::types::modrinth::mods::ModrinthVersionData {
    pub async fn install_mod(&mut self, window: &tauri::Window, author_name: String, destination: &std::path::PathBuf, pack: &mut pack::Modpack) -> Result<SavedModEntry, String> {
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
        let save_entry = SavedModEntry {
            project_id: self.project_id.clone(),
            version_id: self.id.clone(),
            filenames,
            name: self.name.clone(),
            author: author_name
        };
        info!("[debug] Completed download queue for {}", pack_id);
        Ok(save_entry)
    }
}

