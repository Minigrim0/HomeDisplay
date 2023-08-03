use std::env::var;
use crate::api::weather::WeatherInfo;


pub async fn fetch_weather() -> Option<WeatherInfo> {
    let api_key = var("OWM_API_KEY").expect("OWM_API_KEY is required to run this hook").to_string();

    let latitude: f32 = var("OWM_LAT").unwrap_or(
        {
            println!("Using default latitude value (Err: Missing OWM_LAT)");
            "59.0".to_string()
        }
    ).parse::<f32>().unwrap_or(
        {
            println!("Could not convert the given latitude value to f32, using default");
            59.0
        }
    );

    let longitude: f32 = var("OWM_LON").unwrap_or(
        {
            println!("Using default latitude value (Err: Missing OWM_LON)");
            "17.0".to_string()
        }
    ).parse::<f32>().unwrap_or(
        {
            println!("Could not convert the given longitude value to f32, using default");
            17.0
        }
    );

    WeatherInfo::get(latitude, longitude, &api_key).await
}
