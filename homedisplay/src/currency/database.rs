use log::{error, info, warn};
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use crate::models::currency::Conversion;
use crate::traits::Api;

use crate::settings;

#[derive(Serialize, Deserialize)]
struct ConversionDatabase {
    conversion: Conversion,
    freshness: u64,
}

/// Saves the conversion in redis, wrapping it in a struct that includes the freshness of the data
fn store_conversion(conversion: &Conversion, redis_data: &settings::Redis) -> Result<(), String> {
    let conversion: ConversionDatabase = ConversionDatabase {
        conversion: conversion.clone(),
        freshness: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let serialized_conversion: String = match serde_json::to_string(&conversion) {
        Ok(serialized) => serialized,
        Err(error) => {
            return Err(format!(
                "An error occured while serializing the data: {error}",
            ))
        }
    };

    let mut con: redis::Connection = database::get_redis_connection(redis_data)?;

    match con.set::<String, String, redis::Value>(
        "homedisplay:conversion".to_string(),
        serialized_conversion,
    ) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "Could not save serialized data into redis: {error}",
        )),
    }
}

/// Fetches the current conversion from the database, if it is older than a day, data will be refreshed before being returned
pub async fn fetch_current_conversion(
    currency_settings: settings::Currency,
    redis_data: &settings::Redis,
) -> Result<Conversion, String> {
    match database::get_redis_key("homedisplay:conversion".to_string(), redis_data).await {
        Ok(serialized) => match serde_json::from_str(serialized.as_str()) {
            Ok(conversion) => {
                let ConversionDatabase {
                    conversion,
                    freshness,
                } = conversion;
                if SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    - freshness
                    > 86400
                {
                    info!("Data is older than a day, fetching new data from API");
                    match Conversion::api_get(currency_settings).await {
                        Ok(conversion) => {
                            store_conversion(&conversion, redis_data)?;
                            Ok(conversion)
                        }
                        Err(error) => Err(error),
                    }
                } else {
                    info!("Data is fresh enough, returning data from redis");
                    Ok(conversion)
                }
            }
            Err(error) => {
                error!("Could not deserialize the conversion: {error}");
                Err(format!(
                    "An error occured while deserializing the conversion: {error}"
                ))
            }
        },
        Err(err) => {
            warn!("Could not fetch conversion from redis: {}", err);
            info!("Fetching conversion from API");
            match Conversion::api_get(currency_settings).await {
                Ok(conversion) => {
                    info!("Storing conversion in database");
                    store_conversion(&conversion, redis_data)?;
                    Ok(conversion)
                }
                Err(error) => {
                    error!("Could not fetch conversion from API: {}", error);
                    Err(error)
                }
            }
        }
    }
}
