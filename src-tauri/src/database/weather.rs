extern crate redis;
use redis::Commands;
use crate::database::connection::get_redis_connection;
use crate::models::weather::WeatherInfo;


pub fn store_weather(weather: Option<WeatherInfo>) -> Result<WeatherInfo, String> {
    match weather {
        Some(weather) => {
            // Save the weather in redis
            let mut con: redis::Connection = match get_redis_connection() {
                Some(connection) => connection,
                None => return Err("Connection to redis could not be made".to_string())
            };

            let serialized_weather: String = match serde_json::to_string(&weather) {
                Ok(serialized) => serialized,
                Err(error) => format!("An error occured while serializing the data: {}", error)
            };

            match con.set::<String, String, redis::Value>("homedisplay:weather".to_string(), serialized_weather) {
                Ok(_) => Ok(weather),
                Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
            }
        },
        None => Err("Weather information could not be saved, weather is null.".to_string())
    }
}


pub async fn fetch_current_weather() -> Option<WeatherInfo> {
    None
}