/// This file interacts with the database in order to load/store the sites
/// It can also fetch the sites from the API if they are not in the database
use log::{info, warn};
use redis::Commands;
use serde::{Serialize, Deserialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use unidecode::unidecode;

use crate::database;
use crate::traits::{Api, Api1Param};
use common::models::transports::{Site, Departure};


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

/// Stores the site in the database, wrapped in a SiteDatabase struct to store the freshness
/// of the data
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

/// Stores the departures of a site in the database, wrapped in a DepartureDatabase struct
/// to store the freshness of the data
fn store_departures(new_departures: &Vec<Departure>, site: &Site) -> Result<(), String> {
    let departures = DepartureDatabase {
        departures: new_departures.clone(),
        freshness: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };

    let serialized_departures: String = match serde_json::to_string(&departures) {
        Ok(serialized) => serialized,
        Err(error) => return Err(format!("An error occured while serializing the data: {}", error))
    };

    let mut con: redis::Connection = database::get_redis_connection()?;

    match con.set::<String, String, redis::Value>(format!("homedisplay:sites:{}:departures", site.id), serialized_departures) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Could not save serialized data into redis: {}", error))
    }
}

/// Fetches the sites from the API, filters them using the `SL_PLACE_BUS_STOPS` environment variable
/// and stores them in the database
pub async fn fetch_new_sites() -> Result<Vec<Site>, String> {
    // Fecth site names from env
    let name = env::var("SL_PLACE_BUS_STOPS").unwrap_or("".to_string());
    let name: Vec<&str> = name.split(",").collect();
    println!("Filtering on {:?}", name);

    // Fetch sites from the API
    let sites = Site::api_get().await?;
    let mut filtered_sites: Vec<Site> = vec![];
    info!("Fetched {} sites from the API", sites.len());
    for site in sites.iter() {
        if name.is_empty() || name.iter().any(|&n| unidecode(&site.name.to_lowercase()).contains(&unidecode(&n.to_lowercase()))) {
            filtered_sites.push(site.clone());
        }
    }
    info!("Filtered {} sites", filtered_sites.len());

    for site in filtered_sites.iter() {
        store_site(site)?;
    }

    // Store the sites in the database
    Ok(filtered_sites)
}

/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_sites() -> Result<Vec<Site>, String> {
    // Get all sites, filter them and return the list
    let name: String = env::var("SL_PLACE_BUS_STOPS").unwrap_or("".to_string());
    let mut site_list: Vec<Site> = vec![];

    info!("Scanning for sites in the database");
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
            if site_list.is_empty() {
                site_list = fetch_new_sites().await?;

                if site_list.is_empty() {
                    warn!("No sites found in the database, empty list will be returned");
                }
            }
            Ok(site_list)
        },
        Err(e) => Err(e)
    }
}

/// Fetches the current departures from the database, if it is older than a minute,
/// data will be refreshed before being returned
pub async fn get_departures(site: Site) -> Result<Vec<Departure>, String> {
    match database::get_redis_key(format!("homedisplay:sites:{}:departures", site.id)).await {
        Ok(serialized_departures) => {
            let departures = match serde_json::from_str::<DepartureDatabase>(&serialized_departures) {
                Ok(departures) if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - departures.freshness > 60 => {
                    info!("Departures for site {} are older than 60 seconds, fetching new data", site.id);
                    match Departure::api_get(&site).await {
                        Ok(new_departures) => {
                            store_departures(&new_departures, &site)?;
                            new_departures
                        },
                        Err(e) => {
                            warn!("Error while fetching departures: {}", e.to_string());
                            return Err(e);
                        }
                    }
                },
                Ok(departures) => departures.departures,
                Err(e) => {
                    warn!("Error while deserializing departures: {}", e.to_string());
                    return Err(e.to_string());
                }
            };
            Ok(departures)
        },
        Err(e) => {
            warn!("Could not fetch departures from redis: {}", e);
            info!("Fetching new departures from API for site {}", site.id);
            match Departure::api_get(&site).await {
                Ok(new_departures) => {
                    store_departures(&new_departures, &site)?;
                    Ok(new_departures)
                },
                Err(e) => {
                    warn!("Error while fetching departures: {}", e.to_string());
                    Err(e)
                }
            }
        }
    }
}