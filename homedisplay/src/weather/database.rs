use log::{info, warn, error};
use redis::Commands;
use serde::{Serialize, Deserialize};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use crate::settings;
use crate::models::weather::WeatherInfo;
use crate::traits::Api;
use crate::settings::Weather as WeatherSettings;

#[derive(Serialize, Deserialize)]
struct WeatherDatabase {
    weather: WeatherInfo,
    freshness: u64
}

// Saves the weather in redis, wrapping it in a struct that includes the freshness of the data
fn store_weather(weather: &WeatherInfo, redis_data: &settings::Redis) -> Result<(), String> {
    let weather: WeatherDatabase = WeatherDatabase {
        weather: weather.clone(),
        freshness: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };

    let serialized_weather: String = match serde_json::to_string(&weather) {
        Ok(serialized) => serialized,
        Err(error) => return Err(format!("An error occured while serializing the data: {}", error))
    };

    let mut con: redis::Connection = database::get_redis_connection(redis_data)?;

    match con.set::<String, String, redis::Value>("homedisplay:weather".to_string(), serialized_weather) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
    }
}

pub async fn fetch_current_weather(settings: WeatherSettings, redis_data: &settings::Redis) -> Result<WeatherInfo, String> {
    match database::get_redis_key("homedisplay:weather".to_string(), redis_data).await {
        Ok(serialized) => match serde_json::from_str(serialized.as_str()) {
            Ok(conversion) => {
                let WeatherDatabase { weather, freshness } = conversion;
                // Check freshness of current data is less than an hour
                if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - freshness > 3600 {
                    info!("Data is older than an hour, fetching new data from API");
                    match WeatherInfo::api_get(settings).await {
                        Ok(weather) => {
                            store_weather(&weather, redis_data)?;
                            Ok(weather)
                        },
                        Err(error) => Err(error)
                    }
                } else {
                    info!("Data is fresh enough, returning data from redis");
                    Ok(weather)
                }
            },
            Err(error) => {
                error!("Could not deserialize the weather: {}. Fetchin from API", error);
                match WeatherInfo::api_get(settings).await {
                    Ok(weather) => {
                        store_weather(&weather, redis_data)?;
                        Ok(weather)
                    },
                    Err(error) => Err(error)
                }
            }
        },
        Err(err) => {  // If the key does not exist, fetch the data from the API
            warn!("Could not fetch weather from redis: {}", err);
            info!("Fetching weather from the API");
            match WeatherInfo::api_get(settings).await {
                Ok(weather) => {
                    store_weather(&weather, redis_data)?;
                    Ok(weather)
                },
                Err(error) => {
                    error!("Could not fetch weather from the API: {}", error);
                    Err(error)
                }
            }
        }
    }
}
