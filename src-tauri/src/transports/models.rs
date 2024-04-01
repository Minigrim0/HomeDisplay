use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub id: String,
    pub coord: Coordinates,
    pub departures: Vec<Departure>,
    pub timestamp: u64  // Represents the freshness of the data (departures)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Deviation {
    pub importance_level: i32,
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    pub destination: String,
    pub display: String,
    pub line: i32,
}
