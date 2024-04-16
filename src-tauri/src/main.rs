// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod database;
pub mod currency;
pub mod weather;
pub mod transports;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::currency::commands::get_currency,
            crate::weather::commands::get_weather,
            crate::transports::commands::get_site,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
