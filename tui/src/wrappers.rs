use std::time::{SystemTime, Duration};

use common::models::weather::WeatherInfo;


#[derive(Debug)]
pub struct WrappedWeather {
    pub last_refresh: SystemTime,
    pub weather: Result<WeatherInfo, String>,
    pub cooldown: Duration
}


impl WrappedWeather {
    pub fn new(weather: Result<WeatherInfo, String>) -> WrappedWeather {
        let mut w = WrappedWeather::default();
        w.last_refresh = SystemTime::now();
        w.weather = weather;
        w
    }
}

impl Default for WrappedWeather {
    fn default() -> WrappedWeather {
        WrappedWeather {
            last_refresh: SystemTime::now(),
            weather: Err("No weather was fetched yet".to_string()),
            cooldown: Duration::from_secs(30 * 60)
        }
    }
}