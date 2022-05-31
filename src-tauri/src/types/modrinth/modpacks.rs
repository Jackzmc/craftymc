#[allow(unused_imports)]
use log::{info, debug, error, warn};
use futures::stream::StreamExt;
use tokio::io::AsyncWriteExt;

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
    instances_dir: std::path::PathBuf
}

static MAX_CONCURRENT_DOWNLOADS: usize = 4;

impl ModrinthModpackManager {
    pub fn new(instances_folder: std::path::PathBuf) -> ModrinthModpackManager{
        ModrinthModpackManager {
            instances_dir: instances_folder
        }
    }

    async fn download_mod(&self, client: &reqwest::Client, instance_dir: &std::path::Path, entry: ModrinthModpackFileEntry) -> (ModrinthModpackFileEntry, Result<(), String>) {
        debug!("{:?}", std::path::PathBuf::from(instance_dir.to_string_lossy().replace("\\\\", "/")).join(&entry.path));
        let mut file = match tokio::fs::File::create(instance_dir.join(&entry.path)).await {
            Ok(file) => file,
            Err(err) => return (entry, Err(err.to_string()))
        };
        let url = &entry.downloads.as_ref().unwrap()[0];
        debug!("downloading {}, {}", &entry.path, url);
        match client.get(url)
            .send()
            .await
        {
            Ok(mut response) => {
                while let Ok(Some(chunk)) = response.chunk().await {
                    file.write_all(&chunk).await;
                }
                (entry, Ok(()))
            },
            Err(e) => (entry, Err(format!("Failed to download mod: {}", e)))
        }
    }

    pub async fn import(&self, src_folder: &std::path::Path) -> Result<(), String> {
        let manifest_path = src_folder.join("modrinth.index.json");
        let mut modpack: ModrinthModpackManifest = match std::fs::read_to_string(&manifest_path) {
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

        let mut loader: Option<String> = None;
        if modpack.dependencies.forge.is_some() {
            loader = Some("forge".to_string());
            debug!("installing {}-{}", modpack.dependencies.minecraft.as_ref().unwrap(), modpack.dependencies.forge.as_ref().unwrap());
            let installer = match crate::setup::Setup::download_fml_direct(
                src_folder, 
                modpack.dependencies.minecraft.as_ref().unwrap(), 
                modpack.dependencies.forge.as_ref().unwrap()
            ).await {
                Ok(installer) => installer,
                Err(err) => {
                    std::fs::remove_dir_all(src_folder).expect("cleanup failed");
                    return Err(err)
                }
            };

        } else if modpack.dependencies.fabric_loader.is_some() {
            loader = Some("fabric".to_string());
        } else {
            std::fs::remove_dir_all(src_folder).expect("cleanup failed");
            return Err("Unsupported modloader".to_string());
        }
        let client = reqwest::Client::new();
        let clrf = &client;
        // let mut futs = futures::stream::FuturesUnordered::new();
        debug!("downloading {} files", modpack.files.len());
        std::fs::create_dir_all(src_folder.join("mods")).unwrap();
        futures::stream::iter(modpack.files)
            .map(|entry| {
                self.download_mod(clrf, src_folder, entry)
            })
            .buffer_unordered(MAX_CONCURRENT_DOWNLOADS)
            .for_each(|result| async move {
                if let Err(err) = result.1 {
                    debug!("{} failed: {}", result.0.path, err);
                } else {
                    debug!("{} success", result.0.path);
                }
            })
            .await;
        /*while let Some(entry) = modpack.files.pop() {
            futs.push(async move {
                self.download_mod(clrf, src_folder, &entry).await
            });

            if futs.len() == MAX_CONCURRENT_DOWNLOADS {
                futs.next().await.unwrap();
                debug!("while1 iteration");
                
            }
        }

        while let Some(item) = futs.next().await {
            debug!("while2 iteration");
        }*/

        debug!("modloader is: {}", loader.as_ref().unwrap());

        let manifest = crate::pack::Modpack {
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

        let overrides_dir = src_folder.join("overrides");
        if overrides_dir.exists() {
            let mut copy_opts = fs_extra::dir::CopyOptions::new();
            copy_opts.copy_inside = true;
            copy_opts.content_only = true;
            fs_extra::dir::move_dir(overrides_dir, src_folder, &copy_opts).expect("failed to move overrides");
        }

        debug!("writing manifest");
        std::fs::write(src_folder.join("manifest.json"), serde_json::to_string_pretty(&manifest).unwrap()).expect("write manifest failed");
        std::fs::remove_file(manifest_path);
        Ok(())
    }
}