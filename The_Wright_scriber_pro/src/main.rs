#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod db;
mod schema;
mod api;
mod transcription;
mod file_handler;
mod scheduler;
mod config;
mod commands;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::login_user,
            commands::change_password
        ])
        .setup(|app| {
            // Initialization logic here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
