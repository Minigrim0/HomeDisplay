extern crate redis;
use redis::Commands;
use crate::database::connection;
use crate::models::currency::Conversion;


pub fn store_conversion(conversion: Option<Conversion>) -> Result<Conversion, String> {
    match conversion {
        Some(conversion) => {
            // Save the conversion in redis
            let mut con: redis::Connection = match connection::get_redis_connection() {
                Some(connection) => connection,
                None => return Err("Connection to redis could not be made".to_string())
            };

            let serialized_conversion: String = match serde_json::to_string(&conversion) {
                Ok(serialized) => serialized,
                Err(error) => format!("An error occured while serializing the data: {}", error)
            };

            match con.set::<String, String, redis::Value>("homedisplay:conversion".to_string(), serialized_conversion) {
                Ok(_) => Ok(conversion),
                Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
            }
        },
        None => Err("Conversion information could not be saved, conversion is null.".to_string())
    }
}

pub async fn fetch_current_conversion() -> Option<Conversion> {
    match connection::get_redis_key("homedisplay:conversion".to_string()).await {
        Some(serialized) => {
            match serde_json::from_str(serialized.as_str()) {
                Ok(conversion) => Some(conversion),
                Err(error) => {
                    println!("An error occured while deserializing the conversion: {}", error.to_string());
                    None
                }
            }
        },
        None => None
    }
}