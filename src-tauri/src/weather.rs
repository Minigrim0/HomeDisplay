use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    pub coord: Coord,
    pub weather: Vec<Weather>,
}

impl WeatherInfo {
    pub async fn get(latitude: f32, longitude: f32, api_key: &String) -> Option<WeatherInfo> {
        let url: Url = match Url::parse(
            &*format!(
                "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}",
                latitude, longitude, api_key
            )
        ) {
            Ok(url) => url,
            Err(err) => {
                println!("Could not parse URL: {}", err);
                return None;
            }
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(_) => {
                println!("Unable to fetch weather information");
                return None;
            }
        };

        match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<WeatherInfo>().await {
                    Ok(data) => Some(data),
                    Err(err) => {
                        println!("Error while parsing the weather data: {}", err.to_string());
                        None
                    }
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
                None
            },
            _ => {
                println!("Uh oh! Something unexpected happened.");
                None
            },
        }
    }
}
