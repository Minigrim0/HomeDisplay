use reqwest::Url;
use std::env::var;

use super::models::{APIResponse, Conversion};

impl Conversion {
    pub async fn api_get() -> Result<Conversion, String> {
        let api_key: String = match var("OER_API_KEY") {
            Ok(key) => key,
            Err(_) => return Err("Missing API key for Open Exchange Rates, can't fetch new conversion (export OER_API_KEY)".to_string())
        };
    
        let from_currency: String = var("OER_FROM").unwrap_or("EUR".to_string());
        let to_currency: String = var("OER_TO").unwrap_or("SEK".to_string());
    
        let url: Url = match Url::parse(
            &*format!(
                "https://openexchangerates.org/api/latest.json?app_id={}",
                api_key
            )
        ) {
            Ok(url) => url,
            Err(err) => return Err(format!("Could not parse URL: {}", err))
        };
    
        let result = match reqwest::get(url).await {
            Ok(resp) => resp,
            Err(error) => return Err(format!("Error with weather information ! {}", error.to_string()))
        };
    
        match result.status() {
            reqwest::StatusCode::OK => match result.json::<APIResponse>().await {
                Ok(data) => Ok(Conversion::from_base(data, from_currency, to_currency)),
                Err(err) => Err(format!("Error with weather data: {}", err.to_string()))
            },
            reqwest::StatusCode::UNAUTHORIZED => Err("Openexchangerates token is invalid".to_string()),
            _ => Err(format!("Unexpected error ({})", result.status()))
        }
    }
}