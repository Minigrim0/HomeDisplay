use async_trait::async_trait;
use reqwest::Url;
use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::models::currency::Conversion;
use crate::traits::Api;

use crate::settings::Currency as CurrencySettings;

#[derive(Deserialize, Debug)]
/// An API response from Open Exchange Rates
struct APIResponse {
    pub timestamp: i64,
    pub rates: HashMap<String, f32>,
}

trait ConversionFromAPI {
    fn from_base(
        api_response: APIResponse,
        from_currency: String,
        to_currency: String,
    ) -> Conversion;
}

impl ConversionFromAPI for Conversion {
    /// Convert an API response from Open Exchange Rates to a Conversion structure.
    fn from_base(data: APIResponse, from_currency: String, to_currency: String) -> Conversion {
        let from: f32 = match data.rates.get(&from_currency) {
            Some(value) => *value,
            None => 0.0,
        };

        let to: f32 = match data.rates.get(&to_currency) {
            Some(value) => *value,
            None => 0.0,
        };

        Conversion {
            from_currency,
            from_currency_amount: 1.0,
            to_currency,
            to_currency_amount: (to / from),
            timestamp: data.timestamp,
        }
    }
}

#[async_trait]
impl Api<CurrencySettings, Conversion> for Conversion {
    /// Create a conversion structure based on the API response of Open Exchange Rates.
    async fn api_get(currency_settings: CurrencySettings) -> Result<Conversion, String> {
        let url: Url = Url::parse(
            format!(
                "https://openexchangerates.org/api/latest.json?app_id={}",
                currency_settings.api_key
            )
            .as_str(),
        )
        .map_err(|err| format!("Could not parse URL: {}", err))?;

        let result = reqwest::get(url).await.map_err(|error| {
            format!(
                "Error while fetching OpenExchangeRates API: {}",
                error.to_string()
            )
        })?;

        match result.status() {
            reqwest::StatusCode::OK => match result.json::<APIResponse>().await {
                Ok(data) => Ok(Conversion::from_base(
                    data,
                    currency_settings.currency_from,
                    currency_settings.currency_to,
                )),
                Err(err) => Err(format!(
                    "Error while converting Conversion data: {}",
                    err.to_string()
                )),
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                Err("Openexchangerates token is invalid".to_string())
            }
            _ => Err(format!("Unexpected error ({})", result.status())),
        }
    }
}
