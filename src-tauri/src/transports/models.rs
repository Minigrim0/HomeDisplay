/// Simple models for the data we are going to use in the app
/// These models represent the data that is sent to the frontend
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub name: String,
    pub id: String,
    pub coord: Coordinates,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Line {
    pub id: i32,
    pub transport_mode: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Departure {
    pub destination: String,
    pub display: String,
    pub line: Line,
}
