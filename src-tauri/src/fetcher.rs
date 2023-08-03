//!    This executable is used to fetch data from the multiple apis and store
//!    them in redis for caching purposes.
//!    The different structures built by the apis are serialized to a json string
//!    and pushed to redis once per day (Using an external cron to run this executable)

use utils::fetch_weather;

pub mod models;
pub mod api;
pub mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main(){
    let weather = fetch_weather().await;
    println!("Weather is {:?}", weather);
}
