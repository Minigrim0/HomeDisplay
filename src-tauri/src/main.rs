// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models;
pub mod commands;
pub mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![crate::commands::get_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
