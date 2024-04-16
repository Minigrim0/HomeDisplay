use redis::Commands;
use serde::{Serialize, Deserialize};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use super::models::WeatherInfo;


#[derive(Serialize, Deserialize)]
struct WeatherDatabase {
    weather: WeatherInfo,
    freshness: u64
}

// Saves the weather in redis, wrapping it in a struct that includes the freshness of the data
fn store_weather(weather: &WeatherInfo) -> Result<(), String> {
    let weather: WeatherDatabase = WeatherDatabase {
        weather: weather.clone(),
        freshness: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };

    let serialized_weather: String = match serde_json::to_string(&weather) {
        Ok(serialized) => serialized,
        Err(error) => return Err(format!("An error occured while serializing the data: {}", error))
    };

    let mut con: redis::Connection = database::get_redis_connection()?;

    match con.set::<String, String, redis::Value>("homedisplay:weather".to_string(), serialized_weather) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
    }
}

pub async fn fetch_current_weather() -> Result<WeatherInfo, String> {
    match database::get_redis_key("homedisplay:weather".to_string()).await {
        Ok(serialized) => match serde_json::from_str(serialized.as_str()) {
            Ok(conversion) => {
                let WeatherDatabase { weather, freshness } = conversion;
                // Check freshness of current data is less than an hour
                if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - freshness > 3600 {
                    match WeatherInfo::api_get().await {
                        Ok(weather) => {
                            store_weather(&weather)?;
                            Ok(weather)
                        },
                        Err(error) => Err(error)
                    }
                } else {
                    Ok(weather)
                }
            },
            Err(error) => return Err(format!("An error occured while deserializing the weather: {}", error.to_string()))
        },
        Err(err) => Err(err)
    }
}
