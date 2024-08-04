use log::trace;

use common::models::weather::WeatherInfo;
use common::models::transports::{Site, Departure};
use common::models::currency::Conversion;


#[tauri::command]
/// Get the current currency conversion from the database.
pub async fn get_currency() -> Result<Conversion, String> {
    trace!("Currency tauri command invoked");
    common::currency::database::fetch_current_conversion().await
}

#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_sites() -> Result<Vec<Site>, String> {
    trace!("Sites tauri command invoked");
    common::transports::database::get_sites().await
}

#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_departures(site_id: String) -> Result<Vec<Departure>, String> {
    trace!("Departures tauri command invoked");
    common::transports::database::get_departures(site_id).await
}

#[tauri::command]
pub async fn get_weather() -> Result<WeatherInfo, String> {
    trace!("Weather tauri command invoked");
    common::weather::database::fetch_current_weather().await
}
