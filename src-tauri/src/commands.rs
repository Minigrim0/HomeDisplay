use crate::weather::WeatherInfo;
use crate::utils::fetch_weather;

#[tauri::command]
pub async fn get_weather() -> Result<WeatherInfo, ()> {
    match fetch_weather().await {
        Some(weather) => Ok(weather),
        None => Err(())
    }
}
