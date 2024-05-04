use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
/// A conversion between two currencies.
pub struct Conversion {
    pub from_currency: String,
    pub from_currency_amount: f32,
    pub to_currency: String,
    pub to_currency_amount: f32,
    pub timestamp: i64,
}
