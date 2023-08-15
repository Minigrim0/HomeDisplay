//!    This executable is used to fetch data from the multiple apis and store
//!    them in redis for caching purposes.
//!    The different structures built by the apis are serialized to a json string
//!    and pushed to redis once per day (Using an external cron to run this executable)
use colored::Colorize;

use api::{weather, currency, transports};

pub mod models;
pub mod api;
pub mod database;

#[tokio::main(flavor = "multi_thread")]
async fn main(){
    let weather: Option<models::weather::WeatherInfo> = weather::fetch_weather().await;
    match database::weather::store_weather(weather.clone()) {
        Ok(_) => println!("{}", "Weather was successfully stored in the database".green()),
        Err(error) => println!("{}", format!("An error occured while saving the weather informations: {}", error).red())
    };
    let conversion: Option<models::currency::Conversion> = currency::fetch_conversion().await;
    match database::currency::store_conversion(conversion.clone()) {
        Ok(_) => println!("{}", "Conversion information where successfully stored in the database".green()),
        Err(error) => println!("{}", format!("An error occured while save the currency information in the database: {}", error).red())
    };
    let bus_stops: Option<Vec<models::transports::BusStop>> = transports::get_bus_stops().await;
    match database::transports::store_bus_stops(bus_stops.clone()) {
        Ok(stops) => println!("{}", format!("Successfully saved {} new stop(s) in the database", stops.len()).green()),
        Err(error) => println!("{}", format!("An error occured while saving stops information in the database: {}", error).red())
    };
}
