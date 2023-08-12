extern crate redis;
use colored::Colorize;
use crate::models;
use redis::Commands;

fn get_redis_connection() -> Option<redis::Connection> {
    let client = match redis::Client::open(
        format!("redis://{}:{}/",
            std::env::var("REDIS_HOST")
                .expect("This application needs the REDIS_HOST variable to be set"),
            std::env::var("REDIS_PORT")
                .expect("This application needs the REDIS_PORT variable to be set"),
        ))
    {
            Ok(client) => client,
            Err(_) => {
                println!("{}", "Could not connect to redis".red());
                return None;
            }
    };

    match client.get_connection() {
        Ok(connection) => Some(connection),
        Err(error) => {
            println!("{}", format!("Could not connect to redis: {}", error).red());
            return None
        }
    }
}

pub fn store_weather(weather: Option<models::weather::WeatherInfo>) -> Result<models::weather::WeatherInfo, String> {
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

pub fn store_conversion(conversion: Option<models::currency::Conversion>) -> Result<models::currency::Conversion, String> {
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

pub fn store_bus_stops(bus_stops: Option<Vec<models::transports::BusStop>>) -> Result<Vec<models::transports::BusStop>, String> {
    match bus_stops {
        Some(bus_stops) => {
            // Save the weather in redis
            let mut con: redis::Connection = match get_redis_connection() {
                Some(connection) => connection,
                None => return Err("Connection to redis could not be made".to_string())
            };

            let mut error: bool = false;
            for stop in bus_stops.clone() {
                let serialized_stop: String = match serde_json::to_string(&stop) {
                    Ok(serialized) => serialized,
                    Err(error) => format!("An error occured while serializing the data: {}", error)
                };

                // TODO: Try to use the value in the env var as key instead of the real name
                match con.set::<String, String, redis::Value>(format!("homedisplay:stops:{}", stop.name.clone()), serialized_stop) {
                    Ok(_) => println!("{}", format!("Successfully saved stop {}", stop.name).green()),
                    Err(redis_err) => {
                        println!("{}", format!("Could not save serialized stop ({}) into redis: {}", stop.name, redis_err).red());
                        error = true;
                    }
                };
            }

            match error {
                true => Err("An error occured while saving the bus stops".to_string()),
                false => Ok(bus_stops)
            }
        },
        None => Err("Bus stops information could not be saved, bus stops are null.".to_string())
    }
}
