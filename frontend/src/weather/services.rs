use std::time::Duration;
use common::models::weather::WeatherInfo;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use yew::platform::time::sleep;
use crate::glue::get_weather;

const ONE_SEC: Duration = Duration::from_secs(1);
const WEATHER_REFRESH_INTERVAL: Duration = Duration::from_secs(60);


pub fn start_weather_job(weather_ready_cb: Callback<Result<WeatherInfo, String>>) {
    // Spawn a new task that will fetch the weather every 60 seconds
    spawn_local(async move {
        loop {
            // Fetch the weather
            let weather = get_weather().await.unwrap();
        
            // Convert this JsValue object to a WeatherInfo struct
            let weather: Result<WeatherInfo, String> = serde_wasm_bindgen::from_value(weather).map_err(|e| e.to_string());

            // Emit it to the component
            weather_ready_cb.emit(weather);
            sleep(WEATHER_REFRESH_INTERVAL).await;
        }
    });
} 