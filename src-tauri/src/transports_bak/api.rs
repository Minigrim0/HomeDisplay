use colored::Colorize;
use redis::Commands;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env::var;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use crate::transports_bak::models;

/// An answer from the Realtime API endpoint
/// https://transport.integration.sl.se/v1/sites/{SiteId}/departures
#[derive(Serialize, Deserialize, Debug)]
pub struct SiteDepartureAPI {
    pub departures: Vec<models::Departure>,
    pub stop_deviations: Vec<models::Deviation>,
}

/// A site from the API
#[derive(Serialize, Deserialize, Debug)]
pub struct SiteAPI {
    pub id: i32,  // Site id, used to get Realtime departures
    pub name: String,  // The name of the stop
    pub lat: f32,
    pub lon: f32,
}

/// The site list from the API (cache to avoid querying too much)
#[derive(Serialize, Deserialize, Debug)]
pub struct SiteListAPI {
    sites: Vec<SiteAPI>,
    timestamp: u64
}

impl SiteListAPI {
    /// Gets the site list from the API.
    /// This is intended to be cached in redis for later use
    /// (e.g. getting a site's id for the departure API)
    pub async fn get() -> Result<SiteListAPI, String> {
        
    }

    /// Refresh the site list from the API
    async fn refresh() -> Result<SiteListAPI, String> {
        let base_url = "";
        let url: Url = match Url::parse(format!("{}/sites", base_url).as_str()) {
            Ok(url) => url,
            Err(err) => return Err(format!("Could not parse URL: {}", err))
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(err) => return Err(format!("Unable to fetch bus Sites, Err: {}", err))
        };

        let result_status = result.status();
        let result_body = match result.text().await {
            Ok(body) => body,
            Err(err) => return Err(format!("Unable to fetch bus Sites, Err: {}", err))
        };

        match result_status {
            reqwest::StatusCode::OK => {
                match serde_json::from_str::<Vec<SiteAPI>>(&result_body.clone()) {
                    Ok(data) => Ok(SiteListAPI { sites: data}),
                    Err(e) => Err(format!("Error while fetching bus sites: {}", e.to_string())),
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(format!("Unauthorized to fetch bus Sites, check the API key")),
            _ => Err(format!("Uh oh! Something unexpected happened while fetching bus Sites")),
        }
    }

    /// Filters the Site list ase on a stop name and returns the first match
    pub fn filter(&self, stop_name: String) -> Option<&SiteAPI> {
        let filtered: Vec<&SiteAPI> = self.sites.iter().filter(|s| s.name.contains(stop_name.as_str())).collect();
        filtered.first().map_or(None, |s| Some(*s))
    }
}


/// Returns all the departures for all stops stored in the redis database
pub async fn get_all_departures() -> Result<Vec<models::Site>, String> {
    let mut sites_array: Vec<models::Site> = vec![];
    let departures = &mut sites_array;

    match database::scan_iter("homedisplay:stops:*".to_string()).await {
        Ok(stops) => {
            for stop_key in stops.iter() {
                // Fetch the serialized BusStop
                let ser_stop: String = match database::get_redis_key(stop_key.to_string()).await {
                    Ok(ser_stop) => ser_stop,
                    Err(_err) => continue
                };
                // Deserialize it
                let mut stop: models::Site = match serde_json::from_str(ser_stop.as_str()) {
                    Ok(stop) => stop,
                    Err(error) => return Err(format!("Unable to deserialize bus stops, {}", error))
                };

                stop.get_departures().await;
                departures.push(stop);
            }
        },
        Err(err) => return Err(err)
    };

    Ok(sites_array)
}
