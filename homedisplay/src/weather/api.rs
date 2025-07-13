use async_trait::async_trait;
/// Implements the logic for fetching weather data from the OpenWeatherMap API
use reqwest::Url;

use crate::models::weather::WeatherInfo;
use crate::settings::Weather as WeatherSettings;
use crate::traits::Api;

#[async_trait]
impl Api<WeatherSettings, WeatherInfo> for WeatherInfo {
    async fn api_get(weather_settings: WeatherSettings) -> Result<WeatherInfo, String> {
        let url = Url::parse(
            format!(
                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,rain,weather_code,surface_pressure,wind_speed_10m,wind_direction_10m&hourly=temperature_2m,apparent_temperature,precipitation,rain,snowfall,wind_speed_10m,wind_direction_10m&daily=weather_code,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,daylight_duration,uv_index_max&timezone=Europe%2FBerlin",
                weather_settings.latitude, weather_settings.longitude
            ).as_str()
        ).map_err(|err| format!("Could not parse URL: {err}"))?;

        let result = reqwest::get(url)
            .await
            .map_err(|err| format!("Unable to fetch weather information {err}"))?;

        match result.status() {
            reqwest::StatusCode::OK => match result.json::<WeatherInfo>().await {
                Ok(data) => Ok(data),
                Err(err) => Err(format!("Error while parsing the weather data: {err}")),
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(
                "Unauthorized, maybe too much requests have been done for the day ?".to_string(),
            ),
            _ => Err("Uh oh! Something unexpected happened.".to_string()),
        }
    }
}
