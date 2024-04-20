use serde_derive::{Deserialize, Serialize};

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