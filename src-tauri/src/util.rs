use chrono::{DateTime, Utc};

pub fn get_unix_timestamp_now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
}

pub fn get_iso8601(mut timestamp: Option<i64>) -> String {
    if timestamp.is_none() {
        timestamp = Some(get_unix_timestamp_now());
    }
    DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp.unwrap(), 0), Utc).to_rfc3339()
}

pub fn open_folder(path: &std::path::PathBuf) -> Result<(), String> {
    let mut command = match std::env::consts::OS {
        "windows" => std::process::Command::new("explorer"),
        "macos" => std::process::Command::new("open"),
        "linux" => std::process::Command::new("xdg-open"),
        _ => panic!("Unsupported OS")
    };

    match command
        .arg(path)
        .spawn()
    {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err.to_string())
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
