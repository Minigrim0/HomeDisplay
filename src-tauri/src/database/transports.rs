extern crate redis;
use redis::Commands;
use colored::Colorize;
use crate::database::connection::get_redis_connection;
use crate::models::transports::BusStop;


pub fn store_bus_stops(bus_stops: Option<Vec<BusStop>>) -> Result<Vec<BusStop>, String> {
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
