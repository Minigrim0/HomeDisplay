use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    pub coord: Coord,
}

impl WeatherInfo {
    pub async fn get(latitude: f32, longitude: f32, api_key: &String) {
        let url: Url = Url::parse(
            &*format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
                latitude, longitude, api_key
            )
        ).expect("Could not parse URL");

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(_) => {
                println!("Unable to fetch weather information");
                return;
            }
        };

        match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<WeatherInfo>().await {
                    Ok(data) => println!("Noice ! {:?}", data),
                    Err(err) => println!("ohno {}", err.to_string())
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            },
            _ => {
                panic!("Uh oh! Something unexpected happened.");
            },
        };
    }
}
