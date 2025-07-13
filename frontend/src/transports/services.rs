use homedisplay::models::transports::{Departure, Site};
use std::time::Duration;
use futures::stream::{Stream, StreamExt};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use yew::platform::time::interval;
use chrono::{DateTime, Local};

use crate::glue::{get_sites, get_departures};

const ONE_SEC: Duration = Duration::from_secs(1);

pub fn fetch_sites(callback: Callback<Result<Vec<Site>, String>>) {
    spawn_local(async move {
        match get_sites().await {
            Ok(response) => {
                let sites: Result<Vec<Site>, String> = serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string());
                callback.emit(sites);
            },
            Err(e) => {
                callback.emit(serde_wasm_bindgen::from_value(e).map_err(|e| e.to_string()));
            }
        }
    })
}

pub fn fetch_departures(site_id: String, callback: Callback<Result<(String, Vec<Departure>), (String, String)>>) {
    spawn_local(async move {
        match get_departures(site_id.clone()).await {
            Ok(response) => {
                let departures: Result<Vec<Departure>, String> = serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string());
                match departures {
                    Ok(departures) => callback.emit(Ok((site_id, departures))),
                    Err(e) => callback.emit(Err((site_id, e)))
                }
            },
            Err(e) => {
                callback.emit(Err((site_id, serde_wasm_bindgen::from_value(e).unwrap())));
            }
        }
    })
}

/// Returns a stream that emits the current time every second
pub fn stream_time() -> impl Stream<Item = DateTime<Local>> {
    interval(ONE_SEC).map(|_| Local::now())
}
