extern crate redis;
use redis::Commands;
use colored::Colorize;

use crate::database::connection;
use crate::models::transports::{BusStop, StopDepartures};
use crate::api::transports;

pub fn store_bus_stops(bus_stops: Vec<BusStop>) -> Result<Vec<BusStop>, String> {
    // Save the weather in redis
    let mut con: redis::Connection = connection::get_redis_connection()?;

    let mut error: i32 = 0;
    for stop in bus_stops.clone() {
        let serialized_stop: String = match serde_json::to_string(&stop) {
            Ok(serialized) => serialized,
            Err(err) => {
                println!("An error occured while serializing the data: {}", err);
                error += 1;
                continue;
            }
        };

        // TODO: Try to use the value in the env var as key instead of the real name
        match con.set::<String, String, redis::Value>(format!("homedisplay:stops:{}", stop.name.clone()), serialized_stop) {
            Ok(_) => println!("{}", format!("Successfully saved stop {}", stop.name).green()),
            Err(redis_err) => {
                println!("{}", format!("Could not save serialized stop ({}) into redis: {}", stop.name, redis_err).red());
                error += 1;
                continue;
            }
        };
    }

    if error > 0 {
        Err(format!("{} error(s) occured while saving the bus stops", error))
    } else {
        Ok(bus_stops)
    }
}

/**
 * This endpoint fetches Departures directly from the API as realtime is "needed"
 */
pub async fn fetch_current_departures() -> Result<Vec<StopDepartures>, String> {
    transports::get_all_departures().await
}
