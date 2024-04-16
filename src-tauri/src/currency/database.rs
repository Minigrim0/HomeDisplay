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
pub fn store_conversion(conversion: &Conversion) -> Result<(), String> {
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

pub async fn fetch_current_conversion() -> Result<Conversion, String> {
    match database::get_redis_key("homedisplay:conversion".to_string()).await {
        Ok(serialized) => match serde_json::from_str(serialized.as_str()) {
            Ok(conversion) => {
                let ConversionDatabase { conversion, freshness } = conversion;
                // Check freshness of current data is less than a day
                if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - freshness > 86400 {
                    match Conversion::api_get().await {
                        Ok(conversion) => {
                            store_conversion(&conversion)?;
                            Ok(conversion)
                        },
                        Err(error) => Err(error)
                    }
                } else {
                    Ok(conversion)
                }

            },
            Err(error) => Err(format!("An error occured while deserializing the conversion: {}", error.to_string()))
        },
        Err(err) => Err(err)
    }
}
