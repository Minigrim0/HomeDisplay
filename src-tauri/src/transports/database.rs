/// This file interacts with the database in order to load/store the sites
/// It can also fetch the sites from the API if they are not in the database
use crate::database;

use crate::transports::models::Site;


impl Site {
    // Refreshes data in the database by calling the API
    async fn db_refresh() -> Result<Vec<Site>, String> {

        Err("TODO: Implement db_refresh()".to_string())
    }

    // Returns all the sites from the database
    pub async fn db_get_all() -> Result<Vec<Site>, String> {
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
