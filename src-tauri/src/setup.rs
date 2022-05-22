// Will be called AFTER folder structure done
use std::path::PathBuf;
use crate::pack;
use log::{info, debug};

pub struct FirstTimeSetup {
    pub root_folder: PathBuf,
    pub launcher_folder: PathBuf,
    pub downloads_folder: PathBuf
}

impl FirstTimeSetup {
    pub fn new(packs: &pack::ModpackManager) -> FirstTimeSetup {
        FirstTimeSetup {
            root_folder: packs.root_folder.clone(),
            launcher_folder: packs.get_install_folder(),
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
        } else if cfg!(unix) {
            println!("this is unix alike");
        }

        info!("Minecraft launcher install is complete.");
        Ok(())
    }
}