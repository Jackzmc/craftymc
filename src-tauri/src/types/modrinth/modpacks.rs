#[allow(unused_imports)]
use log::{info, debug, error, warn};
use futures::stream::StreamExt;
use tokio::io::AsyncWriteExt;
use sha2::{Sha512, Digest};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpackManifest {
    pub format_version: u8,
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
    pub downloads: Option<Vec<String>>,
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


pub struct ModrinthModpackManager {
    window: tauri::Window
}

static MAX_CONCURRENT_DOWNLOADS: usize = 4;

impl ModrinthModpackManager {
    pub fn new(window: tauri::Window) -> ModrinthModpackManager{
        ModrinthModpackManager {
            window
        }
    }

    pub async fn fetch(project_id: &str) -> Result<ModrinthModpackProject, String> {
        match reqwest::get(format!("https://api.modrinth.com/v2/project/{}", project_id)).await {
            Ok(response) => match response.json::<ModrinthModpackProject>().await {
                Ok(json) => Ok(json),
                Err(err) => return Err(err.to_string())
            },
            Err(err) => return Err(err.to_string())
        }
    }

    async fn download_mod(&self, 
        client: &reqwest::Client, 
        instance_dir: &std::path::Path, 
        entry: ModrinthModpackFileEntry
    ) -> (ModrinthModpackFileEntry, Result<String, String>) {
        let path = instance_dir.join(&entry.path);
        let mut file = match tokio::fs::File::create(&path).await {
            Ok(file) => file,
            Err(err) => return (entry, Err(err.to_string()))
        };
        let url = &entry.downloads.as_ref().unwrap()[0];
        let mut hasher = Sha512::new();
        match client.get(url)
            .send()
            .await
        {
            Ok(mut response) => {
                while let Ok(Some(chunk)) = response.chunk().await {
                    if let Err(err) = file.write_all(&chunk).await {
                        return (entry, Err(err.to_string()));
                    }
                    hasher.update(&chunk);
                }
                let hash = base16ct::lower::encode_string(&hasher.finalize());
                if hash != entry.hashes.sha512 {
                    let err = Err(format!("hashes don't match (got {:?}, expected {}) {}", &hash, &entry.hashes.sha512, &entry.path));
                    return (entry, err);
                }
                (entry, Ok(path.file_name().unwrap().to_str().unwrap().to_string()))
            },
            Err(e) => (entry, Err(format!("Failed to download mod: {}", e)))
        }
    }

    pub async fn import(&self, src_folder: &std::path::Path) -> Result<(), String> {
        let manifest_path = src_folder.join("modrinth.index.json");
        let modpack: ModrinthModpackManifest = match std::fs::read_to_string(&manifest_path) {
            Ok(str) => {
                match serde_json::from_str::<ModrinthModpackManifest>(&str) {
                    Ok(pack) => {
                        pack
                    },
                    Err(err) => {
                        debug!("failed to deserialize: {:?}", err);
                        std::fs::remove_dir_all(src_folder).expect("cleanup failed");
                        return Err(err.to_string())
                    }
                }
            },
            Err(err) => {
                std::fs::remove_dir_all(src_folder).expect("cleanup failed");
                return Err(err.to_string())
            }
        };

        if modpack.game != "minecraft" {
            std::fs::remove_dir_all(src_folder).expect("cleanup failed");
            return Err("Unsupported game".to_string());
        } else if modpack.dependencies.minecraft.is_none() {
            std::fs::remove_dir_all(src_folder).expect("cleanup failed");
            return Err("Missing minecraft version".to_string());
        }

        let loader;
        if modpack.dependencies.forge.is_some() {
            loader = Some("forge".to_string());
        } else if modpack.dependencies.fabric_loader.is_some() {
            loader = Some("fabric".to_string());
        } else {
            std::fs::remove_dir_all(src_folder).expect("cleanup failed");
            return Err("Unsupported modloader".to_string());
        }

        let client = reqwest::Client::new();
        debug!("importing modrinth modpack | modloader: {} | {} files", loader.as_ref().unwrap(), modpack.files.len());
        std::fs::create_dir_all(src_folder.join("mods")).unwrap();

        let mut manifest = crate::pack::Modpack {
            folder_name: Some(src_folder.file_name().unwrap().to_str().unwrap().to_string()),
            name: modpack.name.clone(),
            img_ext: None,
            id: Some(uuid::Uuid::new_v4().to_string()),
            author: None,
            versions: crate::pack::ModpackVersionInfo {
                minecraft: modpack.dependencies.minecraft.unwrap(),
                modloader: modpack.dependencies.forge.or(modpack.dependencies.fabric_loader).unwrap(),
                pack: Some(modpack.version_id)
            },
            lastPlayed: None,
            created: crate::util::get_iso8601(None),
            timesPlayed: None,
            mods: vec![],
            settings: crate::pack::PackSettings {
                javaMemoryMb: 1000,
                useCustomMemory: false,
                javaArgs: None,
                modSource: "modrinth".to_string(),
                modloaderType: loader.unwrap(),
            },
        };

        std::fs::write(src_folder.join("manifest.json"), serde_json::to_string_pretty(&manifest).unwrap()).expect("write manifest failed");
        std::fs::remove_file(manifest_path).unwrap();

        let overrides_dir = src_folder.join("overrides");
        if overrides_dir.exists() {
            let mut copy_opts = fs_extra::dir::CopyOptions::new();
            copy_opts.copy_inside = true;
            copy_opts.content_only = true;
            fs_extra::dir::move_dir(overrides_dir, src_folder, &copy_opts).expect("failed to move overrides");
        }
        let mut optional_mods = Vec::<ModrinthModpackFileEntry>::new();
        let mut required_mods = Vec::<ModrinthModpackFileEntry>::new();

        for entry in modpack.files {
            if let Some(env) = &entry.env {
                match env.client.as_str() {
                    "required" => required_mods.push(entry),
                    "optional" => optional_mods.push(entry),
                    _ => {}
                }
            }
        }

        futures::stream::iter(required_mods)
            .map(|entry| {
                self.download_mod(&client, src_folder, entry)
            })
            .buffer_unordered(MAX_CONCURRENT_DOWNLOADS)
            .for_each(|result| {
                match result.1 {
                    Ok(filename) => {
                        manifest.mods.push(crate::mods::SavedModEntry {
                            name: None,
                            project_id: None,
                            version_id: None,
                            filename,
                            author: None
                        })
                    },
                    Err(err) => debug!("{} failed: {}", result.0.path, err)
                };
                async {

                }
            })
            .await;
            
        if optional_mods.len() > 0 {
            self.window.emit("ask-optional-mods", crate::payloads::OptionalModRequestPayload(optional_mods)).unwrap();
            let (tx, rx) = std::sync::mpsc::sync_channel(32);
            self.window.once("answer-optional-mods", |event| {
                let payload: crate::payloads::OptionalModResponsePayload = serde_json::from_str(event.payload().unwrap()).unwrap();
                tx.send(payload).unwrap();
                drop(tx);
            });
            
            let mods = rx.recv().unwrap().0;
            for entry in mods {
                let result = self.download_mod(&client, &src_folder, entry).await;
                if let Err(err) = result.1 {
                    debug!("{} failed: {}", &result.0.path, err)
                }
            };
        }

        Ok(())
    }
}


#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ModrinthModpackProject {
    pub slug: String,
    pub title: String,
    pub icon_url: Option<String>,
    pub team: String,
}