use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


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

}
