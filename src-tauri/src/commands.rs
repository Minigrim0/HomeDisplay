use log::trace;

use tauri::State;
use std::sync::Mutex;

use common::settings::Settings;
use common::models::weather::WeatherInfo;
use common::models::transports::{Site, Departure};
use common::models::currency::Conversion;


#[tauri::command]
/// Get the current currency conversion from the database.
pub async fn get_currency(settings: State<'_, Mutex<Settings>>) -> Result<Conversion, String> {
    trace!("Currency tauri command invoked");
    let (currency_settings, redis_data) = {
        let settings = match settings.lock() {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };
        (settings.currency.clone(), settings.redis.clone())
    };

    common::currency::database::fetch_current_conversion(currency_settings, &redis_data).await
}

#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_sites(settings: State<'_, Mutex<Settings>>) -> Result<Vec<Site>, String> {
    trace!("Sites tauri command invoked");
    let (stops, redis_data) = {
        let settings = match settings.lock() {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };
        (settings.transports.clone(), settings.redis.clone())
    };
    common::transports::database::get_sites(stops, &redis_data).await
}

#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_departures(settings: State<'_, Mutex<Settings>>, site_id: String) -> Result<Vec<Departure>, String> {
    trace!("Departures tauri command invoked");
    let redis_data = {
        let settings = match settings.lock() {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };
        settings.redis.clone()
    };
    common::transports::database::get_departures(site_id, &redis_data).await
}

#[tauri::command]
pub async fn get_weather(settings: State<'_, Mutex<Settings>>) -> Result<WeatherInfo, String> {
    trace!("Weather tauri command invoked");
    let (weather_settings, redis_data) = {
        let settings = match settings.lock() {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };
        (settings.weather.clone(), settings.redis.clone())
    };

    common::weather::database::fetch_current_weather(weather_settings, &redis_data).await
}
