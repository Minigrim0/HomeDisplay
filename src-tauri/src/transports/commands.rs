use super::database;
use common::models::transports::{Site, Departure};

#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_sites() -> Result<Vec<Site>, String> {
    database::get_sites().await
}


#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_departures(site_id: String) -> Result<Vec<Departure>, String> {
    database::get_departures(site_id).await
}