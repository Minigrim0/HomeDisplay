use super::database;
use super::models::WeatherInfo;

pub async fn get_weather() -> Result<WeatherInfo, String> {
    database::fetch_current_weather().await
}