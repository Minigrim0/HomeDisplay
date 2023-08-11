use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BusStop {
    pub name: String,
    pub site_id: String,
    pub x: String,
    pub y: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PlatsUppSlagAPI {
    pub status_code: i32,
    pub message: Option<String>,
    pub execution_time: i32,
    pub response_data: Vec<BusStop>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Departure {
    pub stop_name: String,
    pub direction: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RealTidAPI {
    pub status_code: i32,
    pub message: Option<String>,
    pub execution_time: i32,
    pub response_data: Vec<Departure>,
}


impl BusStop {
    pub async fn get(api_key: String, base_url: String, bus_stop: String) -> Option<BusStop> {
        let url: Url = match Url::parse(
            &*format!(
                "{}?key={}&searchstring={}",
                base_url, api_key, bus_stop
            )
        ) {
            Ok(url) => url,
            Err(err) => {
                println!("Could not parse URL: {}", err);
                return None;
            }
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(err) => {
                println!("Unable to fetch bus stops, Err: {}", err);
                return None;
            }
        };

        match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<PlatsUppSlagAPI>().await {
                    Ok(data) => {
                        match data.response_data.first() {
                            Some(stop) => Some((*stop).clone()),
                            None => None
                        }
                    },
                    Err(err) => {
                        println!("An error occured while fetching the bus stops: Err {}", err.to_string());
                        None
                    }
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Unauthorized to fetch bus stops, check the API key");
                None
            },
            _ => {
                println!("Uh oh! Something unexpected happened while fetching bus stops");
                None
            },
        }
    }
}
