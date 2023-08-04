use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub short: String,  // e.g. EUR
    pub name: String,  // e.g. Euro
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    pub base_currency: Currency,
    pub base_currency_amount: f32,
    pub to_currency: Currency,
    pub to_currency_amount: f32
}

impl Conversion {
    pub async fn get(from_currency: String, to_currency: String, api_key: String) -> Option<Conversion> {
        let url: Url = match Url::parse(
            &*format!(
                "https://openexchangerates.org/api/latest.json?appid={}",
                api_key
            )
        ) {
            Ok(url) => url,
            Err(err) => {
                println!("Could not parse URL: {}", err);
                return None;
            }
        };

        // Return None by default for now
        None
    }
}
