use reqwest::Url;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env::var;

use super::models::Conversion;

#[derive(Deserialize, Debug)]
/// An API response from Open Exchange Rates
struct APIResponse {
    pub timestamp: i64,
    pub rates: HashMap<String, f32>
}


impl Conversion {
    /// Convert an API response from Open Exchange Rates to a Conversion structure.
    fn from_base(data: APIResponse, from_currency: String, to_currency: String) -> Conversion {
        let from: f32 = match data.rates.get(&from_currency) {
            Some(value) => *value,
            None => 0.0
        };
    
        let to: f32 = match data.rates.get(&to_currency) {
            Some(value) => *value,
            None => 0.0
        };

        Conversion {
            from_currency,
            from_currency_amount: 1.0,
            to_currency,
            to_currency_amount: (to / from),
            timestamp: data.timestamp
        }
    }

    /// Create a conversion structure based on the API response of Open Exchange Rates.
    /// `To` and `From` currencies are taken from the environment variables `OER_FROM` and `OER_TO`.
    /// If these variables are not set, the default values are `EUR` and `SEK`.
    /// The Open Exchange Rates API key is taken from the environment variable as well.
    pub async fn api_get() -> Result<Conversion, String> {
        let api_key: String = match var("OER_API_KEY") {
            Ok(key) => key,
            Err(_) => return Err("Missing API key for Open Exchange Rates (export OER_API_KEY)".to_string())
        };
    
        let from_currency: String = var("OER_FROM").unwrap_or("EUR".to_string());
        let to_currency: String = var("OER_TO").unwrap_or("SEK".to_string());
    
        let url: Url = match Url::parse(
            format!(
                "https://openexchangerates.org/api/latest.json?app_id={}",
                api_key
            ).as_str()
        ) {
            Ok(url) => url,
            Err(err) => return Err(format!("Could not parse URL: {}", err))
        };
    
        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(error) => return Err(format!("Error while fetching OpenExchangeRates API: {}", error.to_string()))
        };
    
        match result.status() {
            reqwest::StatusCode::OK => match result.json::<APIResponse>().await {
                Ok(data) => Ok(Conversion::from_base(data, from_currency, to_currency)),
                Err(err) => Err(format!("Error while converting Conversion data: {}", err.to_string()))
            },
            reqwest::StatusCode::UNAUTHORIZED => Err("Openexchangerates token is invalid".to_string()),
            _ => Err(format!("Unexpected error ({})", result.status()))
        }
    }
}