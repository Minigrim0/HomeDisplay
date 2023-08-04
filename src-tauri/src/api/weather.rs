use crate::models::weather::WeatherInfo;
use std::env::var;


pub async fn fetch_weather() -> Option<WeatherInfo> {
    let api_key = var("OWM_API_KEY")
        .expect("OWM_API_KEY is required to run this hook");

    let latitude: f32 = var("OWM_LAT").unwrap_or(
        {
            println!("Using default latitude value 59.0 (Err: Missing OWM_LAT)");
            "59.0".to_string()
        }
    ).parse::<f32>().unwrap_or(
        {
            println!("Could not convert latitude value to f32, using default");
            59.0
        }
    );

    let longitude: f32 = var("OWM_LON").unwrap_or(
        {
            println!("Using default latitude value 17.0 (Err: Missing OWM_LON)");
            "17.0".to_string()
        }
    ).parse::<f32>().unwrap_or(
        {
            println!("Could not convert longitude value to f32, using default");
            17.0
        }
    );

    WeatherInfo::get(latitude, longitude, &api_key).await
}
