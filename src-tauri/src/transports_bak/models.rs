use serde_derive::{Deserialize, Serialize};

use crate::database;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub id: String,
    pub coord: Coordinates,
    pub departures: Vec<Departure>,
    pub timestamp: u64  // Represents the freshness of the data (departures)
}

impl Site {
    pub async fn get_all() -> Result<Vec<Site>, String> {
        // Fetch all the sites from the database
        // and save them in the database
        let mut site_list: Vec<Site> = vec![];

        if let Ok(sites) = database::scan_iter("homedisplay:sites:*".to_string()).await {
            for site in sites.iter() {
                if let Ok(serialized_site) = database::get_redis_key(site.to_string()).await {
                    if let Ok(site) = serde_json::from_str::<Site>(&serialized_site) {
                        site_list.push(site);
                    }
                }
            }
            Ok(site_list)
        } else {
            // TODO: Try to fetch the sites from the API
            Err("No sites".to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deviation {
    pub importance_level: i32,
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    pub destination: String,
    pub display: String,
    pub line: i32,
}
