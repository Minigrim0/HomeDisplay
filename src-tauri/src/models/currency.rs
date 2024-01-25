use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    pub timestamp: i64,
    pub base: String,
    pub rates: HashMap<String, f32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversion {
    pub from_currency: String,
    pub from_currency_amount: f32,
    pub to_currency: String,
    pub to_currency_amount: f32,
    pub timestamp: i64,
}

impl Conversion {
    pub fn from_base(data: APIResponse, from_currency: String, to_currency: String) -> Conversion {
        let from: f32 = match data.rates.get(&from_currency) {
            Some(value) => *value,
            None => {
                0.0
            }
        };

        let to: f32 = match data.rates.get(&to_currency) {
            Some(value) => *value,
            None => {
                0.0
            }
        };

        Conversion {
            from_currency,
            from_currency_amount: 1.0,
            to_currency,
            to_currency_amount: (to / from),
            timestamp: data.timestamp
        }
    }

    pub async fn get(from_currency: String, to_currency: String, api_key: String) -> Result<Conversion, String> {
        let url: Url = match Url::parse(
            &*format!(
                "https://openexchangerates.org/api/latest.json?app_id={}",
                api_key
            )
        ) {
            Ok(url) => url,
            Err(err) => {
                return Err(format!("Could not parse URL: {}", err));
            }
        };

        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(error) => {
                return Err(format!("Error with weather information ! {}", error.to_string()));
            }
        };

        match result.status() {
            reqwest::StatusCode::OK => {
                match result.json::<APIResponse>().await {
                    Ok(data) => Ok(
                        Conversion::from_base(data, from_currency, to_currency)
                    ),
                    Err(err) => {
                        Err(format!("Error with weather data: {}", err.to_string()))
                    }
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => Err("Openweathermap token is invalid".to_string()),
            _ => Err(format!("Unexpected error ({})", result.status()))
        }
    }
}
