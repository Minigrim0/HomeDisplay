use chrono::{DateTime, Local};
use futures::stream::{Stream, StreamExt};
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use yew::platform::time::{interval, sleep};

use crate::glue::get_currency;
use common::models::currency::Conversion;

const ONE_SEC: Duration = Duration::from_secs(1);
const CONVERSION_REFRESH_INTERVAL: Duration = Duration::from_secs(3600);

pub fn refresh_currency(callback: Callback<Result<Conversion, String>>) {
    spawn_local( async move {
        match get_currency().await {
            Ok(response) => {
                let currency: Result<Conversion, String> = serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string());
                callback.emit(currency);
            },
            Err(e) => {
                callback.emit(Err(serde_wasm_bindgen::from_value(e).unwrap()));
            }
        }
    });
}

pub fn start_currency_job(callback: Callback<Result<Conversion, String>>) {
    spawn_local(async move {
        loop {
            refresh_currency(callback.clone());
            sleep(CONVERSION_REFRESH_INTERVAL).await;
        }
    });
}

pub fn stream_time() -> impl Stream<Item = DateTime<Local>> {
    interval(ONE_SEC).map(|_| Local::now())
}