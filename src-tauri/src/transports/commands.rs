use super::database;
use super::models::{Site, Departure};

#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_sites() -> Result<Vec<Site>, String> {
    database::get_sites().await
}


#[tauri::command]
/// Returns the sites from the database. The list is filtered using elements in the
/// `SL_PLACE_BUS_STOPS` environment variable.
pub async fn get_departures(site: Site) -> Result<Vec<Departure>, String> {
    database::get_departures(site).await
}