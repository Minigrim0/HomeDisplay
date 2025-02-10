/// Implements the logic for fetching weather data from the OpenWeatherMap API
use reqwest::Url;
use std::env::var;
use log::error;
use async_trait::async_trait;

use crate::traits::Api;
use crate::models::weather::WeatherInfo;

const DEFAULT_LATITUDE: f64 = 59.0;
const DEFFAULT_LONGITUDE: f64 = 17.0;

#[async_trait]
impl Api<WeatherInfo> for WeatherInfo {
    async fn api_get() -> Result<WeatherInfo, String> {

        let latitude: f64 = var("WEATHER_LAT")
            .map_err(|e| e.to_string())
            .and_then(|v| v.parse::<f64>()
                .map_err(|e| {
                    error!("Unable to convert `WEATHER_LAT` to float: {}", e.to_string());
                    e.to_string()
                })
            )
            .unwrap_or(DEFAULT_LATITUDE);

        let longitude: f64 = var("WEATHER_LON")
            .map_err(|e| e.to_string())
            .and_then(|v| v.parse::<f64>()
                .map_err(|e| {
                    error!("Unable to convert `WEATHER_LON` to float: {}", e.to_string());
                    e.to_string()
                })
            )
            .unwrap_or(DEFFAULT_LONGITUDE);

        let url = Url::parse(
            format!(
                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,rain,weather_code,surface_pressure,wind_speed_10m,wind_direction_10m&hourly=temperature_2m,apparent_temperature,precipitation,rain,snowfall,wind_speed_10m,wind_direction_10m&daily=weather_code,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,daylight_duration,uv_index_max&timezone=Europe%2FBerlin",
                latitude, longitude
            ).as_str()
        ).map_err(|err| format!("Could not parse URL: {}", err))?;

        let result = reqwest::get(url).await
            .map_err(|err| format!("Unable to fetch weather information {}", err.to_string()))?;

        match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<WeatherInfo>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Error while parsing the weather data: {}", err.to_string()))
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(format!("Unauthorized, maybe too much requests have been done for the day ?")),
            _ => Err("Uh oh! Something unexpected happened.".to_string()),
        }
    }
}
