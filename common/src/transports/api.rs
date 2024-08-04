use reqwest::Url;
use reqwest::header::CONTENT_TYPE;
use serde_derive::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::traits::{Api, Api1Param};
use crate::models::transports::{Coordinates, Departure, Site};


#[derive(Serialize, Deserialize, Debug)]
struct SiteAPI {
    pub id: i32,  // Site id, used to get Realtime departures
    pub name: String,  // The name of the stop

    // Sometimes there is no lat/lon, so we default to 0.0
    #[serde(default)]
    pub lat: Option<f32>,
    #[serde(default)]
    pub lon: Option<f32>,
}


#[async_trait]
impl Api<Vec<Site>> for Site {
    /// Returns the list of all sites from the API
    async fn api_get() -> Result<Vec<Site>, String> {
        let url: Url = match Url::parse("https://transport.integration.sl.se/v1/sites") {
            Ok(url) => url,
            Err(err) => return Err(format!("Could not parse URL: {}", err))
        };

        let client = reqwest::Client::new();
        let result = match client.get(url).header(CONTENT_TYPE, "application/json").send().await {
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
                    Ok(data) => Ok(data.into_iter().map(|site| Site {
                        name: site.name,
                        id: site.id.to_string(),
                        coord: Coordinates {
                            latitude: site.lat.unwrap_or(0.0),
                            longitude: site.lon.unwrap_or(0.0)
                        }
                    }).collect()),
                    Err(e) => Err(format!("Error while fetching bus sites: {}", e.to_string())),
                }
            },
            status => Err(format!("Uh oh! Something unexpected happened while fetching bus sites: {status}")),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct DepartureAPI {
    pub departures: Vec<Departure>
}


#[async_trait]
impl Api1Param<String, Vec<Departure>> for Departure {
    async fn api_get(site_id: String) -> Result<Vec<Departure>, String> {
        let url: Url = match Url::parse(format!("https://transport.integration.sl.se/v1/sites/{}/departures", site_id).as_str()) {
            Ok(url) => url,
            Err(err) => return Err(format!("Could not parse URL: {}", err))
        };

        let client = reqwest::Client::new();
        let result = match client.get(url).header(CONTENT_TYPE, "application/json").send().await {
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
                match serde_json::from_str::<DepartureAPI>(&result_body.clone()) {
                    Ok(data) => Ok(data.departures),
                    Err(e) => Err(format!("Error while fetching departures: {}", e.to_string())),
                }
            },
            status => Err(format!("Uh oh! Something unexpected happened while fetching departures: {status}")),
        }
    }
}