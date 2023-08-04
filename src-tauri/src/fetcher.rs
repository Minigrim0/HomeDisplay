//!    This executable is used to fetch data from the multiple apis and store
//!    them in redis for caching purposes.
//!    The different structures built by the apis are serialized to a json string
//!    and pushed to redis once per day (Using an external cron to run this executable)

use api::{weather, currency};

pub mod models;
pub mod api;

#[tokio::main(flavor = "multi_thread")]
async fn main(){
    let weather = weather::fetch_weather().await;
    let conversion = currency::fetch_conversion().await;
    println!("Weather is {:?}", weather);
    println!("Conversion is {:?}", conversion);
}
