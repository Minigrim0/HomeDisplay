use log::info;

use super::database;
use common::models::weather::WeatherInfo;


#[tauri::command]
pub async fn get_weather() -> Result<WeatherInfo, String> {
    info!("Weather invoked");
    database::fetch_current_weather().await
}
