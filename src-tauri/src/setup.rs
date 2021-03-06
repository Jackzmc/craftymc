// Will be called AFTER folder structure done
use std::path::PathBuf;
use crate::pack;
#[allow(unused_imports)]
use log::{info, debug, error, warn};


pub struct Setup {
    pub root_folder: PathBuf,
    pub launcher_folder: PathBuf,
    pub downloads_folder: PathBuf,
    pub instances_folder: PathBuf,
}

impl Setup {
    pub fn new(packs: &pack::ModpackManager) -> Setup {
        let instance = Setup {
            root_folder: packs.root_folder.clone(),
            launcher_folder: packs.get_install_folder(),
            instances_folder: packs.get_instances_folder(),
            downloads_folder: packs.get_downloads_folder()
        };
        instance
    }

    // Downloads MinecraftInstaller.msi for windows or minecraft-launcher for linux
    pub fn download_launcher(&mut self) -> Result<(), String>  {
        let client = reqwest::blocking::Client::new();
        let installer_name = match std::env::consts::OS {
            "windows" => "MinecraftInstaller.msi",
            "linux" => "Minecraft.tar.gz",
            _ => panic!("Unsupported OS")
        };
        let installer_path = self.downloads_folder.join(&installer_name);
        let mut response = client
            .get(format!("https://launcher.mojang.com/download/{}", &installer_name))
            .send()
            .unwrap();
        let mut dest_file = std::fs::File::create(&installer_path).unwrap();
        std::io::copy(&mut response, &mut dest_file).unwrap();
        dest_file.sync_data().unwrap();
        drop(dest_file);
        debug!("launcher download complete ({}), now installing...", &installer_name);
        self.install_launcher(&installer_name)
    }

    fn install_launcher(&mut self, filename: &str) -> Result<(), String> {
        let src_file = self.downloads_folder.join(filename);
        if cfg!(windows) {
            debug!("msiexec /a {} /qb TARGETDIR=\"{}\"", src_file.to_str().unwrap(), self.launcher_folder.to_str().unwrap());
            let mut status = std::process::Command::new("msiexec")
                .current_dir(&self.downloads_folder)
                .arg("/a")
                .arg(filename)
                .arg("/qn")
                .arg(format!("TARGETDIR={}", self.launcher_folder.to_str().unwrap()))
                // .output()
                .stdin(std::process::Stdio::null())
                .spawn()
                .unwrap();

            // Poll status to check if it's done
            let now = std::time::Instant::now();
            let mut success = false;
            while now.elapsed().as_secs() < 120 {
                match status.try_wait() {
                    Ok(Some(status)) => {
                        if status.success() {
                            success = true;
                            break;
                        } else {
                            return Err(format!("Launcher installation failed with status code {}", status));
                        }
                    },
                    Err(_) => return Err(format!("Launcher was terminated by signal")),
                    _ => continue
                };
            }

            if !success {
                return Err("Installer timed out".to_string())
            }

            debug!("download complete, cleaning up...");
            // Clean up (installer makes it nested for some reason)
            std::fs::rename(
                self.launcher_folder.join("Minecraft Launcher").join("MinecraftLauncher.exe"),
                self.launcher_folder.join("MinecraftLauncher.exe")
            ).expect("mv launcher fail");
            std::fs::remove_dir(self.launcher_folder.join("Minecraft Launcher")).expect("rmdir launcher failed");
            std::fs::remove_file(self.launcher_folder.join("MinecraftInstaller.msi")).expect("rm installer failed");

        } else if cfg!(unix) {
            //tar -xf Minecraft.tar.gz --strip-components=1 -C ../Launcher minecraft-launcher/
            std::process::Command::new("tar")
            .args(["-xf", src_file.to_str().unwrap(), "--strip-components=1", "-C"])
            .arg(self.launcher_folder.to_str().unwrap())
            .status()
            .expect("tar failed");
        }

        let executable = crate::pack::ModpackManager::get_launcher_exec();
        debug!("Running for first time: {}", &executable);
        std::process::Command::new(self.launcher_folder.join(executable))
            .current_dir(&self.launcher_folder)
            .status()
            .expect("Launcher failed to start");

        std::fs::remove_file(src_file).expect("rm src file failed");
        info!("Minecraft launcher install is complete.");
        Ok(())
    }

    pub async fn download_fabric_installer(&self) -> Result<PathBuf, String> {
        let path = self.launcher_folder.join("FabricInstaller.jar");
        match std::fs::File::options().write(true).create_new(true).open(&path) {
            Ok(mut dest_file) => {
                let bytes = reqwest
                    // TODO: Not have a hardcoded version
                    ::get("https://maven.fabricmc.net/net/fabricmc/fabric-installer/0.11.0/fabric-installer-0.11.0.jar")
                    .await.unwrap()
                    .bytes()
                    .await.unwrap();
                let mut content = std::io::Cursor::new(bytes);
                std::io::copy(&mut content, &mut dest_file).unwrap();
                dest_file.sync_data().unwrap();
                debug!("fabric-installer download complete");
            },
            Err(err) => {
                if err.kind() != std::io::ErrorKind::AlreadyExists {
                    error!("error creating dest fabric installer: {}", &err);
                    return Err(err.to_string());
                } else {
                    debug!("skipping download, fabric installer exists already");
                }
            }
        };
        Ok(path)
    }

    pub async fn get_latest_loader(&self, mc_version: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(reqwest::get(format!("https://meta.fabricmc.net/v1/versions/loader/{}", mc_version))
            .await?
            .json::<serde_json::Value>().await?
            .get(0).ok_or("no loaders found")?
            ["version"].to_string()
        )
    }

    pub async fn install_fabric(&self, modpack: &mut pack::Modpack) -> Result<(), String> {
        match self.download_fabric_installer().await {
            Ok(installer_path) => {
                let loader_version = self.get_latest_loader(&modpack.versions.minecraft).await.map_err(|x| x.to_string())?;
                debug!("running: java -jar {:?} -mcversion {:?} -loader {:?} -noprofile -dir {:?}", installer_path, &modpack.versions.minecraft, loader_version, &self.launcher_folder);
                let res = std::process::Command::new("java")
                    .arg("-jar")
                    .arg(&installer_path)
                    .arg("-mcversion")
                    .arg(&modpack.versions.minecraft)
                    .arg("-loader")
                    .arg(&loader_version)
                    .arg("-noprofile")
                    .arg("-dir")
                    .arg(&self.launcher_folder)
                    .status()
                    .unwrap();

                modpack.versions.modloader = loader_version;
                std::fs::remove_file(&installer_path).expect("cleanup installer failed");
                res
            },
            Err(err) => return Err(err) 
        };
        Ok(())
    }
    
    pub async fn download_fml_direct(dest_dir: &std::path::Path, mc_version: &str, forge_version: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let filename = format!("forge-{}-{}-installer.jar", mc_version, forge_version);
        match client
            .get(format!("https://files.minecraftforge.net/maven/net/minecraftforge/forge/{}-{}/{}", mc_version, forge_version, &filename))
            .send()
            .await
        {
            Ok(response) => {
                let mut dest_file = std::fs::File::create(dest_dir.join(&filename)).expect("could not create fml file");
                let mut content = std::io::Cursor::new(response.bytes().await.unwrap());
                std::io::copy(&mut content, &mut dest_file).expect("cp failed");
                dest_file.sync_data().expect("sync failed");
                drop(dest_file);
                debug!("direct fml download complete ({})", &filename);
                Ok(filename)
            },
            Err(err) => return Err(err.to_string())
        }
    }

    async fn download_fml_installer(&self) -> Result<PathBuf, String> {
        let path = self.launcher_folder.join("ForgeCLI.jar");
        debug!("installing to: {:?}", &path);
        match std::fs::File::options().write(true).create_new(true).open(&path) {
            Ok(mut dest_file) => {
                let bytes = reqwest
                    // TODO: Not have a hardcoded version perhaps
                    ::get("https://github.com/TeamKun/ForgeCLI/releases/download/1.0.1/ForgeCLI-1.0.1-all.jar")
                    .await.unwrap()
                    .bytes()
                    .await.unwrap();
                let mut content = std::io::Cursor::new(bytes);
                std::io::copy(&mut content, &mut dest_file).unwrap();
                dest_file.sync_data().unwrap();
                debug!("forge-cli download complete");
            },
            Err(err) => {
                if err.kind() != std::io::ErrorKind::AlreadyExists {
                    error!("error creating dest forge-cli: {}", &err);
                    return Err(err.to_string());
                } else {
                    debug!("skipping download, forge-cli exists already");
                }
            }
        };
        Ok(path)
    }

    pub async fn install_fml(&self, modpack: &mut pack::Modpack) -> Result<(), String> {
        match self.download_fml_installer().await {
            Ok(cli_path) => {
                let installer_path = self.instances_folder
                    .join(modpack.folder_name.as_ref().unwrap())
                    .join(&modpack.versions.modloader);
                debug!("running: java -jar {:?} --installer {:?} --target {:?}", &cli_path, &installer_path, &self.launcher_folder);
                let res = std::process::Command::new("java")
                    .arg("-jar")
                    .arg(cli_path)
                    .arg("--installer")
                    .arg(&installer_path)
                    .arg("--target")
                    .arg(&self.launcher_folder)
                    .status()
                    .unwrap();
                // forge-1.16.5-36.2.35-installer.jar ---> 1.16.5-forge-36.2.3
                modpack.versions.modloader = modpack.versions.modloader.split("-").collect::<Vec<&str>>()[2].to_string();
                
                std::fs::remove_file(&installer_path).expect("cleanup installer failed");
                res
            },
            Err(err) => return Err(err) 
        };
        Ok(())
    }

    pub async fn download_file(dest: &std::path::Path, url: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        match client
            .get(url)
            .send()
            .await
        {
            Ok(response) => {
                let mut dest_file = std::fs::File::create(&dest).expect("could not create file");
                let mut content = std::io::Cursor::new(response.bytes().await.unwrap());
                std::io::copy(&mut content, &mut dest_file).expect("cp failed");
                dest_file.sync_data().expect("sync failed");
                debug!("dl file from ({})", &url);
                Ok(())
            },
            Err(err) => return Err(err.to_string())
        }
    }

    pub async fn watch_for_download() -> Result<std::fs::DirEntry, String> {
        let downloads_dir = &dirs_next::download_dir().expect("cannot find download dir");
        let now = std::time::SystemTime::now();
        while now.elapsed().unwrap().as_secs() < 120 {
          let paths = std::fs::read_dir(downloads_dir).expect("cannot read dir");
          for path in paths {
            let file = path.unwrap();
            match file.metadata().unwrap().created() { 
              Ok(created) => {
                if file.file_type().unwrap().is_file() {
                  if let Ok(duration) = created.duration_since(now) {
                    if duration.as_secs() > 5 && duration.as_secs() <= 30 {
                      let filename = &file.file_name().into_string().unwrap();
                      if filename.ends_with(".jar") {
                        return Ok(file)
                      }
                    }
                  }
                }
              },
              Err(err) => return Err(err.to_string())
            };
          }
          tokio::time::sleep(tokio::time::Duration::from_secs(3)).await
        }
        Err("Watch timed out".to_string())
      }
}