extern crate redis;
use log::trace;
use redis::Commands;

use crate::settings;

pub fn get_redis_connection(redis_settings: &settings::Redis) -> Result<redis::Connection, String> {
    let client = redis::Client::open(
        format!("redis://{}:{}/",
            redis_settings.host,
            redis_settings.port,
        ))
        .map_err(|e| format!("Could not connect to redis.\nIs the database running at the given host & port ?\nError: {}", e))?;

    client.get_connection()
        .map_err(|error| format!("Could not connect to redis: {}", error))
}

pub async fn get_redis_key(key: String, redis_settings: &settings::Redis) -> Result<String, String> {
    trace!("Fetching data from redis with key: {}", key);
    let mut connection = get_redis_connection(redis_settings)?;

    match connection.get::<String, Option<String>>(key) {
        Err(error) => {
            Err(format!("An error occured while fetching the data from redis: {}", error.to_string()))
        },
        Ok(None) => {
            Err("No data stored in the database".to_string())
        },
        Ok(Some(serialized)) => {
            Ok(serialized)
        }
    }
}

pub async fn scan_iter(pattern: String, redis_settings: &settings::Redis) -> Result<Vec<String>, String> {
    let mut connection = get_redis_connection(redis_settings)?;

    let values: Vec<String>;
    match connection.scan_match(pattern) {
        Ok(iterator) => {
            values = iterator.collect();
            Ok(values)
        },
        Err(error) => Err(format!("Unable to get key iterator: {}", error))
    }
}
