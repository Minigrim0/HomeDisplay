use serde_derive::{Deserialize, Serialize};

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
pub struct MainWeatherInfo {
    pub temp: f32,  // Celsius
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,  // hPa
    pub humidity: i32,  // percent
    pub sea_level: i32,
    pub grnd_level: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindInfo {
    pub speed: f32,  // m/s
    pub deg: f32,
    pub gust: f32,  // m/s
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SunInformation {
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub main: MainWeatherInfo,
    pub wind: WindInfo,
    pub sys: SunInformation
}
