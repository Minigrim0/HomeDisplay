use crate::models::weather::WeatherInfo;
use std::env::var;


pub async fn fetch_weather() -> Result<WeatherInfo, String> {
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

    WeatherInfo::get(latitude, longitude, &api_key).await
}
