// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

#[tauri::command]
fn stop_sound() {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap();
    let session = manager.get().unwrap().GetCurrentSession().unwrap();

    session.TryPauseAsync().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![stop_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
