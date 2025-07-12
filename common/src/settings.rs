/// This file contains all the settings structures and functions
/// used to configure the application.

use std::fs;
use serde::{Serialize, Deserialize};
use log::info;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// The settings structure
/// This structure is used to store all the settings of the application
/// It is loaded from a file and can be modified by the user
pub struct Settings {
    pub redis: Redis,
    pub currency: Currency,
    pub weather: Weather,
    pub timezones: Vec<TimezoneData>,
    pub transports: Vec<BusStop>,
}

impl Settings {
    pub fn load_from_file(filename: &str) -> Result<Settings, String> {
        info!("Loading settings from file: {}", filename);
        let content = fs::read_to_string(filename).map_err(|e| e.to_string())?;
        let settings: Settings = toml::from_str(&content).map_err(|e| e.to_string())?;
        Ok(settings)
    }

    /// Dumps this settings structure into a toml string
    pub fn to_string(&self)  -> Result<String, String> {
        toml::to_string(&self).map_err(|e| e.to_string())
    }
}

fn default_lat() -> f64 {
    59.0
}

fn default_lon() -> f64 {
    17.0
}

fn default_display_amount() -> i32 {
    5
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Weather {
    #[serde(default = "default_lat")]
    pub latitude: f64,
    #[serde(default = "default_lon")]
    pub longitude: f64,
    #[serde(default = "default_display_amount")]
    pub display_amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// Structure to hold the timezone data
/// This is used to display the time in different timezones
/// Local time is always displayed
pub struct TimezoneData {
    pub direction: String,  // E or W
    pub offset: f32,  // In hours
    pub name: String,  // A City or a Country (e.g. Paris, France)
}

fn default_from_currency() -> String {
    "SEK".to_string()
}

fn default_to_currency() -> String {
    "EUR".to_string()
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// Structure to hold the currency settings
pub struct Currency {
    pub api_key: String,
    #[serde(default = "default_from_currency")]
    pub currency_from: String,
    #[serde(default = "default_to_currency")]
    pub currency_to: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// Structure to hold the bus stop data
/// This is used to display the bus departures
pub struct BusStop {
    pub name: String,
    pub preffered_lines: Option<Vec<i32>>,
    pub site_id: Option<String>,
}

fn default_redis_host() -> String {
    info!("Using default redis value");
    "localhost".to_string()
}

fn default_redis_port() -> u16 {
    6379
}

fn default_redis_db() -> u8 {
    0
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// Structure to hold the Redis settings
pub struct Redis {
    #[serde(default = "default_redis_host")]
    pub host: String,
    #[serde(default = "default_redis_port")]
    pub port: u16,
    #[serde(default = "default_redis_db")]
    pub db: u8,
}
