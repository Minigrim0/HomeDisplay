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
pub struct PlatsUppSlagAPIError {
    pub status_code: i32,
    pub message: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Departure {
    pub group_of_line: Option<String>,
    pub display_time: String,
    pub line_number: String,
    pub destination: String,
    pub transport_mode: String,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Departures {
    pub latest_update: String,
    pub data_age: i64,
    pub metros: Option<Vec<Departure>>,
    pub buses: Option<Vec<Departure>>,
    pub trams: Option<Vec<Departure>>,
    pub trains: Option<Vec<Departure>>,
    pub ships: Option<Vec<Departure>>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RealTidAPI {
    pub status_code: i64,
    pub message: Option<String>,
    pub execution_time: i64,
    pub response_data: Departures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopDepartures {
    pub stop: BusStop,
    pub departures: Departures
}


impl BusStop {
    pub async fn get(api_key: &String, base_url: &String, bus_stop: &String) -> Result<BusStop, String> {
        let url: Url = match Url::parse(
            &*format!(
                "{}?key={}&searchstring={}",
                base_url, api_key, bus_stop
            )
        ) {
            Ok(url) => url,
            Err(err) => return Err(format!("Could not parse URL: {}", err))
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(err) => return Err(format!("Unable to fetch bus stops, Err: {}", err))
        };

        let result_status = result.status();
        let result_body = match result.text().await {
            Ok(body) => body,
            Err(err) => return Err(format!("Unable to fetch bus stops, Err: {}", err))
        };

        match result_status {
            reqwest::StatusCode::OK => {
                match serde_json::from_str::<PlatsUppSlagAPI>(&result_body.clone()) {
                    Ok(data) => match data.response_data.first() {
                        Some(stop) => Ok((*stop).clone()),
                        None => Err("".to_string())
                    },
                    Err(_err) => match serde_json::from_str::<PlatsUppSlagAPIError>(&result_body) {
                        Ok(error) => Err(format!("An API error occured while fetching the bus stops: {}", error.message.unwrap())),
                        Err(err) => Err(format!("An error occured while fetching the bus stops: {}", err.to_string()))
                    }
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(format!("Unauthorized to fetch bus stops, check the API key")),
            _ => Err(format!("Uh oh! Something unexpected happened while fetching bus stops")),
        }
    }
}


impl RealTidAPI {
    pub async fn get(api_key: &String, base_url: &String, stop: &BusStop) -> Option<RealTidAPI> {
        let url: Url = match Url::parse(
            &*format!(
                "{}?key={}&siteid={}&timewindow={}",
                base_url, api_key, stop.site_id, 60
            )
        ) {
            Ok(url) => url,
            Err(error) => {
                println!("Could not parse realtid URL, {}", error);
                return None
            }
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(error) => {
                println!("Unable to fetch realtid information: {}", error);
                return None
            }
        };

        match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<RealTidAPI>().await {
                    Ok(timings) => Some(timings),
                    Err(error) => {
                        println!("Error while parsing RealTidAPI: {}", error.to_string());
                        None
                    }
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Got unauthorized response while fetching the Departures, check your API key");
                None
            },
            _ => {
                println!("An error occured while fecthing departures !");
                None
            }
        }
    }
}
