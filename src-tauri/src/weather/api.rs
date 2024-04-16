/// Implements the logic for fetching weather data from the OpenWeatherMap API
use reqwest::Url;
use std::env::var;

use super::models::WeatherInfo;

impl WeatherInfo {
    pub async fn api_get() -> Result<WeatherInfo, String> {
        let default_lat = 59.0;
        let default_lon = 17.0;
    
        let api_key = match var("OWM_API_KEY") {
            Ok(k) => k,
            Err(_) => return Err("OWM_API_KEY is required to run this application".to_string())
        };
    
        let latitude: f64 = match var("OWM_LAT") {
            Ok(str_lat) => match str_lat.parse::<f64>() {
                Ok(latitude) => latitude,
                Err(error) => {
                    println!("Could not convert latitude value to f64, using default (Err: {})", error.to_string());
                    default_lat
                }
            },
            Err(_) => {
                println!("Using default latitude value 59.0 (Err: Missing OWM_LAT)");
                default_lat
            }
        };
    
        let longitude: f64 = match var("OWM_LON") {
            Ok(str_lon) => match str_lon.parse::<f64>() {
                Ok(longitude) => longitude,
                Err(error) => {
                    println!("Could not convert longitude value to f64, using default (Err: {})", error.to_string());
                    default_lon
                }
            },
            Err(_) => {
                println!("Using default latitude value 17.0 (Err: Missing OWM_LON)");
                default_lon
            }
        };

        let url: Url = match Url::parse(
            &*format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}",
                latitude, longitude, api_key
            )
        ) {
            Ok(url) => url,
            Err(err) => return Err(format!("Could not parse URL: {}", err))
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(err) => return Err(format!("Unable to fetch weather information {}", err.to_string()))
        };

        match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<WeatherInfo>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Error while parsing the weather data: {}", err.to_string()))
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(format!("Openweather map API key is invalid")),
            _ => Err("Uh oh! Something unexpected happened.".to_string()),
        }
    }
}