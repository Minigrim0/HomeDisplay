use crate::transports::models::StopDepartures;

/// Represents an answer from the Realtime API endpoint
/// https://transport.integration.sl.se/v1/sites/{SiteId}/departures
#[derive(Serialize, Deserialize, Debug)]
pub struct RealTidAPI {
    pub departures: Vec<Departure>,
    pub stop_deviations: Vec<Deviation>,
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
                let res: RealTidAPI = match RealTidAPI::get(&api_key, &base_url, &stop).await {
                    None => {
                        println!("Got no departure information for stop: {}", stop.name);
                        continue;
                    },
                    Some(information) => information
                };

                departures.push(StopDepartures { stop, departures: res.response_data });
            }
        },
        Err(err) => return Err(err)
    };

    Ok(departures_array)
}
