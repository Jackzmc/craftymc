// Will be called AFTER folder structure done
use std::path::PathBuf;
use crate::pack;
use log::{info, debug, error};

pub struct Setup {
    pub root_folder: PathBuf,
    pub launcher_folder: PathBuf,
    pub downloads_folder: PathBuf,
    pub instances_folder: PathBuf
}

impl Setup {
    pub fn new(packs: &pack::ModpackManager) -> Setup {
        Setup {
            root_folder: packs.root_folder.clone(),
            launcher_folder: packs.get_install_folder(),
            instances_folder: packs.get_instances_folder(),
            downloads_folder: packs.get_downloads_folder()
        }
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

            std::process::Command::new("MinecraftLauncher.exe")
                .current_dir(&self.launcher_folder)
                .status()
                .expect("Launcher failed to start");
        } else if cfg!(unix) {
            println!("this is unix alike");
        }

        info!("Minecraft launcher install is complete.");
        Ok(())
    }

    async fn download_fml_installer(&self) -> Result<PathBuf, String> {
        let path = self.launcher_folder.join("ForgeCLI.jar");
        debug!("installing to: {:?}", &path);
        match std::fs::File::options().write(true).create_new(true).open(&path) {
            Ok(mut dest_file) => {
                let bytes = reqwest
                    ::get("https://github.com/TeamKun/ForgeCLI/releases/download/1.0.1/ForgeCLI-1.0.1-all.jar")
                    .await.unwrap()
                    .bytes()
                    .await.unwrap();
                let mut content = std::io::Cursor::new(bytes);
                std::io::copy(&mut content, &mut dest_file).unwrap();
                dest_file.sync_data().unwrap();
                drop(dest_file);
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
                    if duration.as_secs() > 5 && duration.as_secs() <= 12 {
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
        }
        Err("Watch timed out".to_string())
      }
}