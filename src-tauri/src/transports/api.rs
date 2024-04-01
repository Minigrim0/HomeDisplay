use colored::Colorize;
use redis::Commands;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env::var;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use crate::transports::models;

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
    sites: Vec<SiteAPI>
}

impl models::Site {
    // Saves the site in redis, updating timestamps
    async fn save(&mut self) -> Result<(), String> {
        let mut con: redis::Connection = database::get_redis_connection()?;

        self.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("wtf time went backwards").as_secs();

        let serialized_stop: String = match serde_json::to_string(&self) {
            Ok(serialized) => serialized,
            Err(err) => return Err(format!("An error occured while serializing the data: {}", err))
        };

        match con.set::<String, String, redis::Value>(format!("homedisplay:stops:{}", self.id), serialized_stop) {
            Ok(_) => {
                println!("{}", format!("Successfully saved stop {}", self.name).green());
                Ok(())
            },
            Err(redis_err) => Err(format!("{}", format!("Could not save serialized stop ({}) into redis: {}", self.name, redis_err).red()))
        }
    }

    /// Refreshes data by calling the API endpoint
    /// The refreshed data is sent to the redis database
    async fn refresh(&mut self) -> Result<(), String> {
        let base_url = match var("SL_TRANSPORTS_ROOT_URL") {
            Ok(b) => b,
            Err(e) => return Err(e.to_string())
        };

        let url: Url = match Url::parse(
            &*format!("{}/sites/{}/departures", base_url, self.id)
        ) {
            Ok(url) => url,
            Err(error) => {
                return Err(format!("Could not parse realtid URL, {}", error));
            }
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(error) => {
                return Err(format!("Unable to fetch realtid information: {}", error))
            }
        };

        let timings = match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<SiteDepartureAPI>().await {
                    Ok(timings) => timings,
                    Err(error) => return Err(format!("Error while parsing SiteDepartureAPI: {}", error.to_string()))
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(format!("Got unauthorized response while fetching the Departures, check your API key"))
            },
            _ => {
                return Err(format!("An error occured while fecthing departures !"))
            }
        };

        self.departures = timings.departures;

        Ok(())
    }

    /// Check the freshness of the data in the redis database
    /// If above a certain threshold, data is refreshed from the
    /// API and the new departures Are stored in the database
    pub async fn get_departures(&mut self) -> Result<&Vec<models::Departure>, String> {
        // Check freshness
        if SystemTime::now().duration_since(UNIX_EPOCH).expect("wtf time went backwards").as_secs() - self.timestamp < 60 {  // Refresh every minute
            self.refresh();
        }

        Ok(&self.departures)
    }
}


impl SiteListAPI {
    /// Gets the site list from the API.
    /// This is intended to be cached in redis for later use (e.g. getting a site's id for the
    /// departure API)
    pub async fn get(base_url: &String) -> Result<SiteListAPI, String> {
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
pub async fn get_all_departures() -> Result<Vec<models::SiteDepartures>, String> {
    let base_url = match var("SL_REALTIME_ROOT_URL") {
        Ok(url) => url,
        Err(_) => {
            return Err("Missing Root URL for SL's realtid, can't fetch site ids\nexport SL_REALTIME_ROOT_URL".to_string());
        }
    };

    let mut departures_array: Vec<models::SiteDepartures> = vec![];
    let departures = &mut departures_array;

    match database::scan_iter("homedisplay:stops:*".to_string()).await {
        Ok(stops) => {
            for stop_key in stops.iter() {
                // Fetch the serialized BusStop
                let ser_stop: String = match database::get_redis_key(stop_key.to_string()).await {
                    Ok(ser_stop) => ser_stop,
                    Err(_err) => continue
                };
                // Deserialize it
                let stop: models::Site = match serde_json::from_str(ser_stop.as_str()) {
                    Ok(stop) => stop,
                    Err(error) => return Err(format!("Unable to deserialize bus stops, {}", error))
                };

                // Fetch departures for this stop
                let res: SiteDepartureAPI = match SiteDepartureAPI::get(&base_url, &stop).await {
                    None => {
                        println!("Got no departure information for stop: {}", stop.name);
                        continue;
                    },
                    Some(information) => information
                };

                departures.push(models::SiteDepartures { stop, departures: res.departures });
            }
        },
        Err(err) => return Err(err)
    };

    Ok(departures_array)
}
