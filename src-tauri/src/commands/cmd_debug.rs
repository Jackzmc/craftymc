use crate::setup::Setup;
use crate::AppState;
  
#[tauri::command]
pub fn debug_install_launcher(state: tauri::State<'_, AppState>) {
  let mut setup = Setup::new(&state.modpacks.blocking_lock());
  setup.download_launcher().expect("download launcher failed");
}
