use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    temperature: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl WeatherInfo {
    fn get(symbol: &String, api_key: &String) {
        let url: String = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&*url);
        match reqwest::get(url) {
            Ok(data) => match data.json::<WeatherInfo>() {
                Ok(json_data) => json_data,
                Err(err) => {
                    println!("An error occured ! {}", err.to_string());
                }
            },
            Err(err) => {
                println!("Error while calling the API ! {}", err.to_string());
            }
        };
    }
}
