use crate::models::weather::WeatherInfo;
use reqwest::Url;

impl WeatherInfo {
    pub async fn get(latitude: f32, longitude: f32, api_key: &String) -> Option<WeatherInfo> {
        let url: Url = match Url::parse(
            &*format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}",
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
