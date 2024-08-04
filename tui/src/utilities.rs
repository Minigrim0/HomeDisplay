use std::time::SystemTime;

use crate::app::App;
use crate::wrappers::WrappedWeather;

use common::weather::database::fetch_current_weather;


pub fn refresh_weather() -> WrappedWeather {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build() {
            Ok(rt) => rt,
            Err(e) => {
                let mut weather: WrappedWeather = WrappedWeather::default();
                weather.weather = Err(format!("Unable to build a tokio runtime to fetch the weather {}", e.to_string()));
                return weather
        }
    };

    match rt.block_on(fetch_current_weather()) {
        Ok(weather) => WrappedWeather::new(Ok(weather)),
        Err(e) => WrappedWeather::new(Err(e.to_string()))
    }
}
