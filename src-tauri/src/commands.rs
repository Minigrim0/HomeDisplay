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
