// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;


fn main() {
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_currency,
            commands::get_weather,
            commands::get_sites,
            commands::get_departures,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
