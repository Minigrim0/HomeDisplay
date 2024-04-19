use log::info;

use super::database;
use super::models::Conversion;


#[tauri::command]
/// Get the current currency conversion from the database.
pub async fn get_currency() -> Result<Conversion, String> {
    info!("Currency invoked");
    database::fetch_current_conversion().await
}
