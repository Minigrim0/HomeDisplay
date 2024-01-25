use crate::models::currency::Conversion;
use std::env::var;

pub async fn fetch_conversion() -> Result<Conversion, String> {
    let api_key: String = var("OER_API_KEY")
        .expect("OER_API_KEY is required to run this hook");

    let from_currency: String = var("OER_FROM").unwrap_or(
        {
            "EUR".to_string()
        }
    );

    let to_currency: String = var("OER_TO").unwrap_or(
        {
            "SEK".to_string()
        }
    );

    Conversion::get(from_currency, to_currency, api_key).await
}
