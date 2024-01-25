extern crate redis;
use redis::Commands;
use crate::database::connection;
use crate::models::weather::WeatherInfo;


pub fn store_weather(weather: WeatherInfo) -> Result<WeatherInfo, String> {
    // Save the weather in redis
    let mut con: redis::Connection = connection::get_redis_connection()?;

    let serialized_weather: String = match serde_json::to_string(&weather) {
        Ok(serialized) => serialized,
        Err(error) => format!("An error occured while serializing the data: {}", error)
    };

    match con.set::<String, String, redis::Value>("homedisplay:weather".to_string(), serialized_weather) {
        Ok(_) => Ok(weather),
        Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
    }
}

pub async fn fetch_current_weather() -> Result<WeatherInfo, String> {
    match connection::get_redis_key("homedisplay:weather".to_string()).await {
        Ok(serialized) => match serde_json::from_str(serialized.as_str()) {
            Ok(conversion) => Ok(conversion),
            Err(error) => return Err(format!("An error occured while deserializing the weather: {}", error.to_string()))
        },
        Err(err) => Err(err)
    }
}
