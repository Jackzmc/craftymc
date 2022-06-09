use std::process::Child;

use chrono::{DateTime, Utc};
#[allow(unused_imports)]
use log::{info, debug, error, warn};

pub fn get_unix_timestamp_now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
}

pub fn get_iso8601(mut timestamp: Option<i64>) -> String {
    if timestamp.is_none() {
        timestamp = Some(get_unix_timestamp_now());
    }
    DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp.unwrap(), 0), Utc).to_rfc3339()
}

pub fn open_folder(path: &std::path::Path) -> Result<Child, String> {
    let mut command = match std::env::consts::OS {
        "windows" => {
            let mut cmd = std::process::Command::new("explorer");
            cmd.arg("/select,").arg(path);
            cmd
        },
        "macos" => {
            let mut cmd = std::process::Command::new("open");
            cmd.arg(path);
            cmd
        },
        "linux" => {
            let mut cmd = std::process::Command::new("xdg-open");
            cmd.arg(path);
            cmd
        }, 
        // Linux: Use 'dbus-send --session --print-reply --dest=org.freedesktop.FileManager1 --type=method_call /org/freedesktop/FileManager1 org.freedesktop.FileManager1.ShowItems array:string:"file:///home/me/path/to/folder/or/file" string:"' ?
        _ => return Err("Unsupported OS".to_string())
    };

    command.spawn().map_err(|err| err.to_string())
}

pub fn mv_as_admin(src_path: &std::path::Path, dest_path: &std::path::PathBuf) {
    let src = src_path.to_str().unwrap();
    let dest = dest_path.to_str().unwrap();
    match std::env::consts::OS {
        "windows" => {
            let status = runas::Command::new(r"C:\Windows\System32\cmd.exe")
                .gui(true)
                .arg("/c")
                .arg("copy")
                .arg(src)
                .arg(dest)
                .status()
                .unwrap();
            debug!("cmd /c copy {:?} {:?} returned: status {:?}", src, dest, status.code().unwrap_or(-1));

            let status = runas::Command::new(r"C:\Windows\System32\cmd.exe")
                .gui(true)
                .arg("/s")
                .arg("/c")
                .arg(format!("del \"{}\"", src))
                .status()
                .unwrap();
            debug!("cmd /c \"del \"{:?}\"\" returned: status {:?}", src, status.code().unwrap_or(-1))
        },
        "linux" => {
            runas::Command::new(r"/bin/sh")
                .gui(true)
                .arg(format!("'cp \"{}\" \"{}\"; rm \"{}\"'", src, dest, src))
                .status()
                .unwrap();
        },
        _ => panic!("Unsupported OS")
    }
}

pub fn get_directory_tree(path: &std::path::Path) -> TreeEntry {
    _recurse_dir(path, path)
}
fn _recurse_dir(root: &std::path::Path, path: &std::path::Path) -> TreeEntry {
    let files = std::fs::read_dir(path).unwrap();
    let mut entries = Vec::new();
    for path in files {
        let file = path.unwrap();
        let filetype = file.file_type().unwrap();
        if filetype.is_dir() {
            entries.push(_recurse_dir(root, &file.path()));
        } else if filetype.is_file() {
            entries.push(TreeEntry {
                name: file.file_name().to_str().unwrap().to_string(),
                children: None,
                selected: false,
                id: file.path().strip_prefix(root).ok().unwrap().to_str().unwrap().to_string()
            });
        }
    }
    TreeEntry {
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        children: Some(entries.into_boxed_slice()),
        selected: false,
        id: path.strip_prefix(root).ok().unwrap().to_str().unwrap().to_string()
    }
}


#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TreeEntry {
    name: String,
    children: Option<Box<[TreeEntry]>>,
    selected: bool, // Used on ui
    id: String
}
