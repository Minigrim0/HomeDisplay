use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Site {
    pub name: String,
    pub site_id: String,
    pub x: String,
    pub y: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Deviation {
    pub importance_level: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Line {
    pub id: i32,
    pub designation: String,
    pub transport_mode: String,
}

/// A departure (must be linked to a Site)
#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    pub destination: String,
    pub display: String,
    pub line: Line,
}

/// A Site with all its departures
#[derive(Serialize, Deserialize, Debug)]
pub struct SiteDepartures {
    pub stop: Site,
    pub departures: Vec<Departure>
}


