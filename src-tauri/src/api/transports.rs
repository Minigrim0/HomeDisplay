extern crate redis;
use std::env::var;

use crate::models::transports::{BusStop, RealTidAPI, StopDepartures};
use crate::database::connection;


pub async fn check_bus_stop(stop_name: String) -> Result<BusStop, String> {
    match connection::get_redis_key(format!("homedisplay:{}", stop_name)).await {
        Ok(place_id) => match serde_json::from_str(&place_id) {
            Ok(value) => Ok(value),
            Err(error) => Err(format!("Could not deserialize busstop from redis: {}\nIs the data malformed ?", error))
        },
        Err(_) => Err("Could not find the bus stop in the database".to_string())
    }
}


pub async fn get_bus_stops() -> Result<Vec<BusStop>, String> {
    let api_key: String = match var("SL_PLACE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Err("Missing API key for SL's platsuppslag, can't fetch new busstops (export SL_PLACE_API_KEY)".to_string());
        }
    };

    let root_url: String = match var("SL_PLACE_ROOT_URL") {
        Ok(url) => url,
        Err(_) => {
            return Err("Missing Root URL for SL's platsuppslag, can't fetch site ids (export SL_PLACE_ROOT_URL)".to_string());
        }
    };

    let bus_stop_list: String;
    let bus_stops: Vec<&str> = match var("SL_PLACE_BUS_STOPS") {
        Ok(stops) => {
            bus_stop_list = stops.clone();
            bus_stop_list.split(",").collect::<Vec<&str>>()
        },
        Err(_) => {
            return Err("Missing bus stops, can't define what to fetch (export SL_PLACE_BUS_STOPS)".to_string());
        }
    };

    let mut bus_stops_array: Vec<BusStop> = vec![];
    let stops: &mut Vec<BusStop> = &mut bus_stops_array;
    for stop in bus_stops.iter() {
        match check_bus_stop(stop.to_string()).await {
            Ok(place_id) => stops.push(place_id),  // The bus stop is cached in redis
            Err(_) => {  // The bus stop is not in redis, fetch it from the API
                match BusStop::get(api_key.clone(), root_url.clone(), (*stop).to_string()).await {
                    Ok(bus_stop) => stops.push(bus_stop),
                    Err(_) => continue
                }
            }
        }
    };

    Ok(bus_stops_array)
}


pub async fn get_all_departures() -> Result<Vec<StopDepartures>, String> {
    let api_key: String = match var("SL_REALTIME_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Err("Missing API key for SL realtid API. Can't fetch departures".to_string());
        }
    };

    let base_url = match var("SL_REALTIME_ROOT_URL") {
        Ok(url) => url,
        Err(_) => {
            return Err("Missing Root URL for SL's realtid, can't fetch site ids\nexport SL_REALTIME_ROOT_URL".to_string());
        }
    };

    let mut departures_array: Vec<StopDepartures> = vec![];
    let departures = &mut departures_array;

    match connection::scan_iter("homedisplay:stops:*".to_string()).await {
        Ok(stops) => {
            for stop_key in stops.iter() {
                // Fetch the serialized BusStop
                let ser_stop: String = match connection::get_redis_key(stop_key.to_string()).await {
                    Ok(ser_stop) => ser_stop,
                    Err(_err) => continue
                };
                // Deserialize it
                let stop: BusStop = match serde_json::from_str(ser_stop.as_str()) {
                    Ok(stop) => stop,
                    Err(error) => return Err(format!("Unable to deserialize bus stops, {}", error))
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
        Err(err) => return Err(err)
    };

    Ok(departures_array)
}
