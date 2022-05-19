#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod Settings;

struct AppState {
  settings: Settings::Settings,
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> Settings::Settings {
  state.settings.clone()
}


fn main() {
  let mut settings = Settings::Settings::new();
  tauri::Builder::default()
    .manage(AppState{settings})
    .invoke_handler(tauri::generate_handler![get_settings])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
