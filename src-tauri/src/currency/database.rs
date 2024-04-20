use log::{error, info, warn};
use redis::Commands;
use serde::{Serialize, Deserialize};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use super::models::Conversion;

#[derive(Serialize, Deserialize)]
struct ConversionDatabase {
    conversion: Conversion,
    freshness: u64
}

/// Saves the conversion in redis, wrapping it in a struct that includes the freshness of the data
fn store_conversion(conversion: &Conversion) -> Result<(), String> {
    let conversion: ConversionDatabase = ConversionDatabase {
        conversion: conversion.clone(),
        freshness: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };

    let serialized_conversion: String = match serde_json::to_string(&conversion) {
        Ok(serialized) => serialized,
        Err(error) => return Err(format!("An error occured while serializing the data: {}", error))
    };

    let mut con: redis::Connection = database::get_redis_connection()?;

    match con.set::<String, String, redis::Value>("homedisplay:conversion".to_string(), serialized_conversion) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
    }
}

/// Fetches the current conversion from the database, if it is older than a day, data will be refreshed before being returned
pub async fn fetch_current_conversion() -> Result<Conversion, String> {
    match database::get_redis_key("homedisplay:conversion".to_string()).await {
        Ok(serialized) => match serde_json::from_str(serialized.as_str()) {
            Ok(conversion) => {
                let ConversionDatabase { conversion, freshness } = conversion;
                if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - freshness > 86400 {
                    info!("Data is older than a day, fetching new data from API");
                    match Conversion::api_get().await {
                        Ok(conversion) => {
                            store_conversion(&conversion)?;
                            Ok(conversion)
                        },
                        Err(error) => Err(error)
                    }
                } else {
                    info!("Data is fresh enough, returning data from redis");
                    Ok(conversion)
                }

            },
            Err(error) => {
                error!("Could not deserialize the conversion: {}", error);
                Err(format!("An error occured while deserializing the conversion: {}", error.to_string()))
            }
        },
        Err(err) => {
            warn!("Could not fetch conversion from redis: {}", err);
            info!("Fetching conversion from API");
            match Conversion::api_get().await {
                Ok(conversion) => {
                    info!("Storing conversion in database");
                    store_conversion(&conversion)?;
                    Ok(conversion)
                },
                Err(error) =>{
                    error!("Could not fetch conversion from API: {}", error);
                    Err(error)
                }
            }
        }
    }
}
