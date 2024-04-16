use super::database;
use super::models::Conversion;


#[tauri::command]
pub async fn get_currency() -> Result<Conversion, String> {
    database::fetch_current_conversion().await
}