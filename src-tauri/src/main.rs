// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use log::error;
use clap::Parser;
use tauri::{Builder, Manager};
use common::settings::Settings;

mod commands;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author="Minigrim0", version, about="The Tauri version of home-display")]
struct Args {
    #[arg(short = 's', long, default_value = "settings.toml")]
    /// Path to the settings file
    settings: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let settings = match Settings::load_from_file(&args.settings) {
        Ok(settings) => settings,
        Err(err) => {
            error!("Failed to load settings: {}", err);
            std::process::exit(1);
        }
    };

    Builder::default()
        .setup(|app| {
              app.manage(Mutex::new(settings));
              Ok(())
            })
        .invoke_handler(tauri::generate_handler![
            commands::get_currency,
            commands::get_weather,
            commands::get_sites,
            commands::get_departures,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
