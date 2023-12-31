// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use deep_spotify::app_state::App;
use deep_spotify::commands;

fn main() {
    tauri::Builder::default()
        .manage(App::default())
        .invoke_handler(tauri::generate_handler![
            commands::file_dialog::load_user_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
