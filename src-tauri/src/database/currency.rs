extern crate redis;
use redis::Commands;
use crate::database::connection;
use crate::models::currency::Conversion;


pub fn store_conversion(conversion: Conversion) -> Result<Conversion, String> {
    // Save the conversion in redis
    let mut con: redis::Connection = connection::get_redis_connection()?;

    let serialized_conversion: String = match serde_json::to_string(&conversion) {
        Ok(serialized) => serialized,
        Err(error) => format!("An error occured while serializing the data: {}", error)
    };

    match con.set::<String, String, redis::Value>("homedisplay:conversion".to_string(), serialized_conversion) {
        Ok(_) => Ok(conversion),
        Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
    }
}

pub async fn fetch_current_conversion() -> Result<Conversion, String> {
    match connection::get_redis_key("homedisplay:conversion".to_string()).await {
        Ok(serialized) => {
            match serde_json::from_str(serialized.as_str()) {
                Ok(conversion) => Ok(conversion),
                Err(error) => {
                    Err(
                        format!(
                            "An error occured while deserializing the conversion: {}",
                            error.to_string()
                        )
                    )
                }
            }
        },
        Err(err) => Err(err)
    }
}
