extern crate redis;
use std::env::var;
use redis::Commands;

use crate::models::transports::{BusStop, RealTidAPI, StopDepartures};
use crate::database::connection;


pub fn check_bus_stop(stop_name: String) -> Option<BusStop> {
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
                return None
            }
    };

    let mut con = match client.get_connection() {
        Ok(connection) => connection,
        Err(_) => {
            return None
        }
    };

    match con.get::<String, String>(format!("homedisplay:{}", stop_name)) {
        Ok(place_id) => Some(match serde_json::from_str(&place_id) {
            Ok(value) => value,
            Err(error) => {
                println!("Could not deserialize busstop from redis {}", error);
                return None
            }
        }),
        Err(_) => None
    }
}


pub async fn get_bus_stops() -> Option<Vec<BusStop>> {
    let api_key: String = match var("SL_PLACE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Missing API key for SL's platsuppslag, can't fetch new busstops (export SL_PLACE_API_KEY)");
            return None;
        }
    };

    let root_url: String = match var("SL_PLACE_ROOT_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("Missing Root URL for SL's platsuppslag, can't fetch site ids (export SL_PLACE_ROOT_URL)");
            return None;
        }
    };

    let bus_stop_list: String;
    let bus_stops: Vec<&str> = match var("SL_PLACE_BUS_STOPS") {
        Ok(stops) => {
            bus_stop_list = stops.clone();
            bus_stop_list.split(",").collect::<Vec<&str>>()
        },
        Err(_) => {
            println!("Missing bus stops, can't define what to fetch (export SL_PLACE_BUS_STOPS)");
            return None;
        }
    };

    let mut bus_stops_array: Vec<BusStop> = vec![];
    let stops: &mut Vec<BusStop> = &mut bus_stops_array;
    for stop in bus_stops.iter() {
        match check_bus_stop(stop.to_string()) {
            Some(place_id) => stops.push(place_id),  // The bus stop is cached in redis
            None => {  // The bus stop is not in redis, fetch it from the API
                match BusStop::get(api_key.clone(), root_url.clone(), (*stop).to_string()).await {
                    Some(bus_stop) => stops.push(bus_stop),
                    None => println!()
                }
            }
        }
    };

    Some(bus_stops_array)
}


pub async fn get_all_departures() -> Option<Vec<StopDepartures>> {
    let api_key: String = match var("SL_REALTIME_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Missing API key for SL realtid API. Can't fetch departures");
            return None;
        }
    };

    let base_url = match var("SL_REALTIME_ROOT_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("Missing Root URL for SL's realtid, can't fetch site ids (export SL_REALTIME_ROOT_URL)");
            return None;
        }
    };

    let mut departures_array: Vec<StopDepartures> = vec![];
    let departures = &mut departures_array;

    match connection::scan_iter("homedisplay:stops:*".to_string()).await {
        Some(stops) => {
            for stop_key in stops.iter() {
                // Fetch the serialized BusStop
                let ser_stop: String = match connection::get_redis_key(stop_key.to_string()).await {
                    Some(ser_stop) => ser_stop,
                    None => continue
                };
                // Deserialize it
                let stop: BusStop = match serde_json::from_str(ser_stop.as_str()) {
                    Ok(stops) => stops,
                    Err(error) => {
                        println!("Unable to deserialize bus stops, {}", error);
                        return None
                    }
                };

                // Fetch departures for this stop
                let res: RealTidAPI = match RealTidAPI::get(api_key.clone(), base_url.clone(), stop.clone()).await {
                    None => {
                        println!("Got no departure information for stop: {}", stop.name);
                        continue;
                    },
                    Some(information) => information
                };

                departures.push(StopDepartures { stop: stop, departures: res.response_data });
            }
        },
        None => {
            println!("Unable to fetch bus stops from redis: key is none");
            return None
        }
    };

    Some(departures_array)
}