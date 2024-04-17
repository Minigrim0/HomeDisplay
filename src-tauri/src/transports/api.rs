use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

use super::models::{Coordinates, Departure, Site};


#[derive(Serialize, Deserialize, Debug)]
struct SiteAPI {
    pub id: i32,  // Site id, used to get Realtime departures
    pub name: String,  // The name of the stop
    pub lat: f32,
    pub lon: f32,
}


impl Site {
    /// Returns the list of all sites from the API
    pub async fn api_get() -> Result<Vec<Site>, String> {
        let url: Url = match Url::parse("https://transport.integration.sl.se/v1/sites") {
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
                    Ok(data) => Ok(data.into_iter().map(|site| Site {
                        name: site.name,
                        id: site.id.to_string(),
                        coord: Coordinates {
                            latitude: site.lat,
                            longitude: site.lon
                        }
                    }).collect()),
                    Err(e) => Err(format!("Error while fetching bus sites: {}", e.to_string())),
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(format!("Unauthorized to fetch bus Sites, check the API key")),
            _ => Err(format!("Uh oh! Something unexpected happened while fetching bus Sites")),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct DepartureAPI {
    pub departures: Vec<Departure>
}

impl Departure {
    pub async fn api_get(site: Site) -> Result<Vec<Departure>, String> {
        let url: Url = match Url::parse(format!("https://transport.integration.sl.se/v1/sites/{}/departures", site.id).as_str()) {
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
                match serde_json::from_str::<DepartureAPI>(&result_body.clone()) {
                    Ok(data) => Ok(data.departures),
                    Err(e) => Err(format!("Error while fetching departures: {}", e.to_string())),
                }
            },
            status => Err(format!("Uh oh! Something unexpected happened while fetching bus Sites: {status}")),
        }
    }
}