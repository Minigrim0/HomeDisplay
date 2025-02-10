use std::time::Duration;
use chrono::{DateTime, Local};
use futures::stream::{Stream, StreamExt};
use common::models::weather::WeatherInfo;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use yew::platform::time::{interval, sleep};
use crate::glue::get_weather;

const ONE_SEC: Duration = Duration::from_secs(1);
const WEATHER_REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 30);

pub fn refresh_weather(callback: Callback<Result<WeatherInfo, String>>) {
    spawn_local( async move {
        match get_weather().await {
            Ok(response) => {
                let weather: Result<WeatherInfo, String> = serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string());
                callback.emit(weather);
            },
            Err(e) => {
                callback.emit(serde_wasm_bindgen::from_value(e).map_err(|e| e.to_string()));
            }
        }
    });
}

pub fn start_weather_job(callback: Callback<Result<WeatherInfo, String>>) {
    // Spawn a new task that will fetch the weather every 60 seconds
    spawn_local(async move {
        loop {
            // Fetch the weather
            refresh_weather(callback.clone());

            sleep(WEATHER_REFRESH_INTERVAL).await;
        }
    });
}

/// Returns a stream that emits the current time every second
pub fn stream_time() -> impl Stream<Item = DateTime<Local>> {
    interval(ONE_SEC).map(|_| Local::now())
}
