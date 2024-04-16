use colored::Colorize;

use crate::api::{weather, currency};

use crate::models;
use crate::database;

use crate::transports;

#[tauri::command]
pub async fn get_weather() -> Result<models::weather::WeatherInfo, String> {
    database::weather::fetch_current_weather().await
}


#[tauri::command]
pub async fn get_currency() -> Result<models::currency::Conversion, String> {
    database::currency::fetch_current_conversion().await
}


#[tauri::command]
pub async fn get_departures() -> Result<Vec<transports::models::Site>, String> {
    transports::commands::fetch_current_departures().await
}

#[tauri::command]
pub async fn fetch_apis(){
    match weather::fetch_weather().await {
        Ok(weather) => match database::weather::store_weather(&weather) {
            Ok(_) => println!("{}", "Weather was successfully stored in the database".green()),
            Err(error) => println!("{}", format!("An error occured while saving the weather informations: {}", error).red())
        },
        Err(err) => println!("{}", format!("No weather information were saved: {}", err).red())
    }

    match currency::fetch_conversion().await {
        Ok(conversion) => match database::currency::store_conversion(&conversion) {
            Ok(_) => println!("{}", "Conversion information where successfully stored in the database".green()),
            Err(error) => println!("{}", format!("An error occured while save the currency information in the database: {}", error).red())
        },
        Err(error) => println!("{}", format!("No currency information were saved: {}", error).red())
    }

    match transports::api::SiteListAPI::get().await {
        Ok(bus_stops) => match transports::commands::store_bus_stops(&bus_stops) {
            Ok(()) => println!("{}", format!("Successfully saved {} new stop(s) in the database", bus_stops.len()).green()),
            Err(error) => println!("{}", format!("An error occured while saving stops information in the database: {}", error).red())
        },
        Err(error) => println!("{}", format!("No bus stop information were saved: {}", error).red())
    }
}
