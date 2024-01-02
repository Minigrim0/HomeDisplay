use colored::Colorize;

use crate::api::{weather, currency, transports};

use crate::models;
use crate::database;

#[tauri::command]
pub async fn get_weather() -> Result<models::weather::WeatherInfo, ()> {
    match database::weather::fetch_current_weather().await {
        Some(weather) => Ok(weather),
        None => Err(())
    }
}


#[tauri::command]
pub async fn get_currency() -> Result<models::currency::Conversion, String> {
    match database::currency::fetch_current_conversion().await {
        Some(currency) => Ok(currency),
        None => Err("No currency conversion could be found".to_string())
    }
}


#[tauri::command]
pub async fn get_departures() -> Result<Vec<models::transports::StopDepartures>, String> {
    match database::transports::fetch_current_departures().await {
        Some(departures) => Ok(departures),
        None => Err("No departures could be found".to_string())
    }
}

#[tauri::command]
pub async fn fetch_apis(){
    let weather: Option<models::weather::WeatherInfo> = weather::fetch_weather().await;
    match database::weather::store_weather(weather.clone()) {
        Ok(_) => println!("{}", "Weather was successfully stored in the database".green()),
        Err(error) => println!("{}", format!("An error occured while saving the weather informations: {}", error).red())
    };
    let conversion: Option<models::currency::Conversion> = currency::fetch_conversion().await;
    match database::currency::store_conversion(conversion.clone()) {
        Ok(_) => println!("{}", "Conversion information where successfully stored in the database".green()),
        Err(error) => println!("{}", format!("An error occured while save the currency information in the database: {}", error).red())
    };
    let bus_stops: Option<Vec<models::transports::BusStop>> = transports::get_bus_stops().await;
    match database::transports::store_bus_stops(bus_stops.clone()) {
        Ok(stops) => println!("{}", format!("Successfully saved {} new stop(s) in the database", stops.len()).green()),
        Err(error) => println!("{}", format!("An error occured while saving stops information in the database: {}", error).red())
    };
}