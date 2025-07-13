/// This file interacts with the database in order to load/store the sites
/// It can also fetch the sites from the API if they are not in the database
use log::{info, warn};
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use unidecode::unidecode;

use crate::database;
use crate::models::transports::{Departure, Site};
use crate::traits::Api;

use crate::settings;

#[derive(Serialize, Deserialize)]
struct SiteDatabase {
    site: Site,
    freshness: u64,
}

#[derive(Serialize, Deserialize)]
struct DepartureDatabase {
    departures: Vec<Departure>,
    freshness: u64,
}

/// Stores the site in the database, wrapped in a SiteDatabase struct to store the freshness
/// of the data
fn store_site(site: &Site, redis_data: &settings::Redis) -> Result<(), String> {
    let site: SiteDatabase = SiteDatabase {
        site: site.clone(),
        freshness: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let serialized_site: String = match serde_json::to_string(&site) {
        Ok(serialized) => serialized,
        Err(error) => {
            return Err(format!(
                "An error occured while serializing the data: {}",
                error
            ))
        }
    };

    let mut con: redis::Connection = database::get_redis_connection(redis_data)?;

    match con.set::<String, String, redis::Value>(
        format!("homedisplay:sites:{}", site.site.id),
        serialized_site,
    ) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "Could not save serialized data into redis: {}",
            error
        )),
    }
}

/// Stores the departures of a site in the database, wrapped in a DepartureDatabase struct
/// to store the freshness of the data
fn store_departures(
    new_departures: &Vec<Departure>,
    site_id: &str,
    redis_data: &settings::Redis,
) -> Result<(), String> {
    let departures = DepartureDatabase {
        departures: new_departures.clone(),
        freshness: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let serialized_departures: String = match serde_json::to_string(&departures) {
        Ok(serialized) => serialized,
        Err(error) => {
            return Err(format!(
                "An error occured while serializing the data: {}",
                error
            ))
        }
    };

    let mut con: redis::Connection = database::get_redis_connection(redis_data)?;

    match con.set::<String, String, redis::Value>(
        format!("homedisplay:sites:{}:departures", site_id),
        serialized_departures,
    ) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "Could not save serialized data into redis: {}",
            error
        )),
    }
}

/// Fetches the sites from the API, filters them using the transports settings
/// and stores them in the database
pub async fn fetch_new_sites(
    stops: &Vec<settings::BusStop>,
    redis_data: &settings::Redis,
) -> Result<Vec<Site>, String> {
    info!("Filtering on:");
    for stop in stops.iter() {
        info!(" - {}", stop.name);
    }

    // Fetch sites from the API
    let sites = Site::api_get(()).await?;
    let mut filtered_sites: Vec<Site> = vec![];
    info!("Fetched {} sites from the API", sites.len());
    for site in sites.iter() {
        if stops.is_empty()
            || stops.iter().any(|s| {
                unidecode(&site.name.to_lowercase()).contains(&unidecode(&s.name.to_lowercase()))
                    && s.site_id.as_ref().and_then(|id| Some(id == &site.id)) != Some(false)
            })
        {
            filtered_sites.push(site.clone());
        }
    }
    info!("Filtered {} sites", filtered_sites.len());

    for site in filtered_sites.iter() {
        store_site(site, redis_data)?;
    }

    // Store the sites in the database
    Ok(filtered_sites)
}

/// Returns the sites from the database. The list is filtered using elements in the
/// stops vector from the settings
pub async fn get_sites(
    stops: &Vec<settings::BusStop>,
    redis_data: &settings::Redis,
) -> Result<Vec<Site>, String> {
    // Get all sites, filter them and return the list
    let mut site_list: Vec<Site> = vec![];

    info!("Scanning for sites in the database");
    let sites = database::scan_iter("homedisplay:sites:*".to_string(), redis_data).await?;
    for site in sites.iter() {
        if let Ok(serialized_site) = database::get_redis_key(site.to_string(), redis_data).await {
            let site = match serde_json::from_str::<SiteDatabase>(&serialized_site) {
                Ok(site)
                    if SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        - site.freshness
                        > 30 * 86400 =>
                {
                    info!(
                        "Site {} is older than 30 days, fetching new data",
                        site.site.id
                    );
                    match Site::api_get(()).await {
                        Ok(new_sites) => {
                            let mut site_to_return: Option<Site> = None;
                            for new_site in new_sites.into_iter() {
                                store_site(&new_site, redis_data)?;
                                if new_site.id == site.site.id {
                                    site_to_return = Some(new_site);
                                }
                            }
                            if let Some(site) = site_to_return {
                                site
                            } else {
                                warn!(
                                    "Could not find current site ({}) in new sites",
                                    site.site.id
                                );
                                continue;
                            }
                        }
                        Err(e) => {
                            warn!("Error while fetching bus sites: {}", e.to_string());
                            continue;
                        }
                    }
                }
                Ok(site) => site.site,
                Err(e) => {
                    warn!("Error while deserializing bus sites: {}", e.to_string());
                    continue;
                }
            };
            let site_name = &site.name.to_lowercase();
            if stops.is_empty()
                || stops.iter().any(|stop| {
                    unidecode(site_name).contains(&unidecode(&stop.name.to_lowercase()))
                        && stop.site_id.as_ref().and_then(|id| Some(id == &site.id)) != Some(false)
                })
            {
                site_list.push(site);
            }
        } else {
            warn!("Could not fetch site from redis");
        }
    }
    if site_list.is_empty() {
        site_list = fetch_new_sites(stops, redis_data).await?;

        if site_list.is_empty() {
            warn!("No sites found in the database, empty list will be returned");
        }
    }
    Ok(site_list)
}

/// Fetches the current departures from the database, if it is older than a minute,
/// data will be refreshed before being returned
pub async fn get_departures(
    site_id: String,
    redis_data: &settings::Redis,
) -> Result<Vec<Departure>, String> {
    match database::get_redis_key(
        format!("homedisplay:sites:{}:departures", site_id),
        redis_data,
    )
    .await
    {
        Ok(serialized_departures) => {
            let departures = match serde_json::from_str::<DepartureDatabase>(&serialized_departures)
            {
                Ok(departures)
                    if SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        - departures.freshness
                        > 60 =>
                {
                    info!(
                        "Departures for site {} are older than 60 seconds, fetching new data",
                        site_id
                    );
                    match Departure::api_get(site_id.clone()).await {
                        Ok(new_departures) => {
                            store_departures(&new_departures, &site_id, redis_data)?;
                            new_departures
                        }
                        Err(e) => {
                            warn!("Error while fetching departures: {}", e.to_string());
                            return Err(e);
                        }
                    }
                }
                Ok(departures) => departures.departures,
                Err(e) => {
                    warn!("Error while deserializing departures: {}", e.to_string());
                    return Err(e.to_string());
                }
            };
            Ok(departures)
        }
        Err(e) => {
            warn!("Could not fetch departures from redis: {}", e);
            info!("Fetching new departures from API for site {}", site_id);
            match Departure::api_get(site_id.clone()).await {
                Ok(new_departures) => {
                    store_departures(&new_departures, &site_id, redis_data)?;
                    Ok(new_departures)
                }
                Err(e) => {
                    warn!("Error while fetching departures: {}", e.to_string());
                    Err(e)
                }
            }
        }
    }
}
