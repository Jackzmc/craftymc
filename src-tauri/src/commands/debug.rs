use crate::setup::Setup;
use crate::AppState;
  
#[tauri::command]
fn install_launcher(state: tauri::State<'_, AppState>) {
  let mut setup = Setup::new(&state.modpacks.blocking_lock());
  setup.download_launcher().expect("download launcher failed");
}

#[tauri::command]
fn echo(msg: &str) {
    println!("echo! {}", msg);
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
  tauri::plugin::Builder::new("debug")
    .invoke_handler(tauri::generate_handler![install_launcher, echo])
    .build()
}

