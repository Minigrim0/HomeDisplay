use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use crate::transports_bak::models;


/// Get the sites from the database. If the data 
/// is not in the database, fetch it from the API
pub async fn get_sites() -> Result<Vec<models::Site>, String> {
    if let Ok(freshness) = database::get_redis_key("homedisplay:sites:timestamp".to_string()).await {
        if let Ok(last_refreshed) = freshness.parse::<u64>() {
            if  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - last_refreshed > 60 {
                // The data is older than 60 seconds, fetch new data
                // and update the timestamp
                Err("TODO refresh data".to_string())
            } else {
                Ok(models::Site::get_all().await?)  // Get all the sites from the database
            }
        } else {
            // Freshness could not be fetched from the database
            // Get all the data from the API and save a new timestamp
            Err("Unable to parse freshness timestamp".to_string())
        }
    } else {
        // Freshness could not be fetched from the database
        // Get all the data from the API and save a new timestamp
        Err("Not implemeneted yet".to_string())
    }
}
