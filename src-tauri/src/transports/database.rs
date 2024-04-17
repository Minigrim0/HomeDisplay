/// This file interacts with the database in order to load/store the sites
/// It can also fetch the sites from the API if they are not in the database
use log::{info, warn};
use redis::Commands;
use serde::{Serialize, Deserialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use unidecode::unidecode;

use crate::database;
use crate::transports::models::{Site, Departure};


#[derive(Serialize, Deserialize)]
struct SiteDatabase {
    site: Site,
    freshness: u64
}

#[derive(Serialize, Deserialize)]
struct DepartureDatabase {
    departures: Vec<Departure>,
    freshness: u64
}

fn store_site(site: &Site) -> Result<(), String> {
    let site: SiteDatabase = SiteDatabase {
        site: site.clone(),
        freshness: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };

    let serialized_site: String = match serde_json::to_string(&site) {
        Ok(serialized) => serialized,
        Err(error) => return Err(format!("An error occured while serializing the data: {}", error))
    };

    let mut con: redis::Connection = database::get_redis_connection()?;

    match con.set::<String, String, redis::Value>(format!("homedisplay:sites:{}", site.site.id), serialized_site) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
    }
}

fn store_departures(site: &Site) -> Result<(), String> {
    Ok(())
}

/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_sites() -> Result<Vec<Site>, String> {
    // Get all sites, filter them and return the list
    let name: String = env::var("SL_PLACE_BUS_STOPS").unwrap_or("".to_string());
    let mut site_list: Vec<Site> = vec![];

    match database::scan_iter("homedisplay:sites:*".to_string()).await {
        Ok(sites) => {
            for site in sites.iter() {
                if let Ok(serialized_site) = database::get_redis_key(site.to_string()).await {
                    let site = match serde_json::from_str::<SiteDatabase>(&serialized_site) {
                        Ok(site) if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - site.freshness > 30 * 86400 => {
                            info!("Site {} is older than 30 days, fetching new data", site.site.id);
                            match Site::api_get().await {
                                Ok(new_sites) => {
                                    let mut site_to_return: Option<Site> = None;
                                    for new_site in new_sites.into_iter() {
                                        store_site(&new_site)?;
                                        if new_site.id == site.site.id {
                                            site_to_return = Some(new_site);
                                        }
                                    }
                                    if let Some(site) = site_to_return {
                                        site
                                    } else {
                                        warn!("Could not find current site ({}) in new sites", site.site.id);
                                        continue;
                                    }
                                },
                                Err(e) => {
                                    warn!("Error while fetching bus sites: {}", e.to_string());
                                    continue;
                                }
                            }
                        },
                        Ok(site) => site.site,
                        Err(e) => {
                            warn!("Error while deserializing bus sites: {}", e.to_string());
                            continue;
                        }
                    };
                    if name.is_empty() || unidecode(&site.name.to_lowercase()).contains(&unidecode(&name.to_lowercase())) {
                        site_list.push(site);
                    }
                } else {
                    warn!("Could not fetch site from redis");
                }
            }
            Ok(site_list)
        },
        Err(e) => Err(e)
    }
}

/// Fetches the current departures from the database, if it is older than a minute,
/// data will be refreshed before being returned
pub async fn get_departures() -> Result<Vec<Departure>, String> {
    Err("Not implemented yet".to_string())
}