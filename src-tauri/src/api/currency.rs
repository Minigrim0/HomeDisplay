use crate::models::currency::Conversion;
use std::env::var;

pub async fn fetch_conversion() -> Result<Conversion, String> {
    let api_key: String = match var("OER_API_KEY") {
        Ok(key) => key,
        Err(_) => return Err("Missing API key for Open Exchange Rates, can't fetch new conversion (export OER_API_KEY)".to_string())
    };

    let from_currency: String = var("OER_FROM").unwrap_or("EUR".to_string());
    let to_currency: String = var("OER_TO").unwrap_or("SEK".to_string());

    Conversion::get(from_currency, to_currency, api_key).await
}
