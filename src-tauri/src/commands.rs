use crate::weather::{Coord, WeatherInfo};
use crate::utils::fetch_weather;

#[tauri::command]
pub async fn get_weather() -> Result<WeatherInfo, ()> {
    fetch_weather().await;
    Ok(WeatherInfo {
        coord: Coord {lat: 12.0, lon: 12.0},
    })
}
