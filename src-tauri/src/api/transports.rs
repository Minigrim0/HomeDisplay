use crate::models::transports::BusStop;
use std::env::var;


// TODO: Add a check to see if bus stop is already in redis
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
            println!("Missing bus stops, can't define what to fetch (export SL_PLACE_API_KEY)");
            return None;
        }
    };

    let mut bus_stops_array: Vec<BusStop> = vec![];
    let stops = &mut bus_stops_array;
    for stop in bus_stops.iter() {
        match BusStop::get(api_key.clone(), root_url.clone(), (*stop).to_string()).await {
            Some(bus_stop) => stops.push(bus_stop),
            None => println!()
        }
    };

    Some(bus_stops_array)
}

