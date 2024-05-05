use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::{AttrValue, Callback};
use yew::platform::time::sleep;

use crate::glue::get_weather;

const ONE_SEC: Duration = Duration::from_secs(1);
const WEATHER_REFRESH_INTERVAL: Duration = Duration::from_secs(60);


pub fn start_weather_job(weather_ready_cb: Callback<AttrValue>) {
    // Spawn a background task that will fetch a joke and send it to the component.
    spawn_local(async move {
        loop {
            // Fetch the online joke
            let weather = get_weather().await.unwrap();

            // Emit it to the component
            weather_ready_cb.emit(AttrValue::from(weather.as_string().unwrap()));
            sleep(WEATHER_REFRESH_INTERVAL).await;
        }
    });
} 