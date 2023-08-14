extern crate redis;
use redis::Commands;
use crate::database::connection::get_redis_connection;
use crate::models::currency::Conversion;


pub fn store_conversion(conversion: Option<Conversion>) -> Result<Conversion, String> {
    match conversion {
        Some(conversion) => {
            // Save the conversion in redis
            let mut con: redis::Connection = match get_redis_connection() {
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
    None
}