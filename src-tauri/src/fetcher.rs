//!    This executable is used to fetch data from the multiple apis and store
//!    them in redis for caching purposes.
//!    The different structures built by the apis are serialized to a json string
//!    and pushed to redis once per day (Using an external cron to run this executable)

use api::{weather, currency, transports};

pub mod models;
pub mod api;

#[tokio::main(flavor = "multi_thread")]
async fn main(){
    let weather: Option<models::weather::WeatherInfo> = weather::fetch_weather().await;
    let conversion: Option<models::currency::Conversion> = currency::fetch_conversion().await;
    let bus_stops: Option<Vec<models::transports::BusStop>> = transports::get_bus_stops().await;
    println!("Weather is {:?}", weather);
    println!("Conversion is {:?}", conversion);
    println!("Bus stops are {:?}", bus_stops);
}
