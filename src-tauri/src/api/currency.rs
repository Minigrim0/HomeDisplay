use crate::models::currency::Conversion;
use std::env::var;

pub async fn fetch_conversion() -> Option<Conversion> {
    let api_key: String = var("OER_API_KEY")
        .expect("OER_API_KEY is required to run this hook");

    let from_currency: String = var("OER_FROM").unwrap_or(
        {
            println!("Using EUR as default conversion currency (Err: Missing OER_FROM)");
            "EUR".to_string()
        }
    );

    let to_currency: String = var("OER_TO").unwrap_or(
        {
            println!("Using SEK as default conversion currency (Err: Missing OER_TO)");
            "SEK".to_string()
        }
    );

    Conversion::get(from_currency, to_currency, api_key).await
}
