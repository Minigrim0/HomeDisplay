use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainWeatherInfo {
    pub temp: f64,  // Celsius
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,  // hPa
    pub humidity: i32,  // percent
    pub sea_level: Option<i32>,
    pub grnd_level: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindInfo {
    pub speed: f64,  // m/s
    pub deg: f64,
    pub gust: Option<f64>,  // m/s
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SunInformation {
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherInfo {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub main: MainWeatherInfo,
    pub wind: WindInfo,
    pub sys: SunInformation
}

impl WeatherInfo {
    pub async fn get(latitude: f64, longitude: f64, api_key: &String) -> Result<WeatherInfo, String> {
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
