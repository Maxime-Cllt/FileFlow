use serde::Deserialize;

#[derive(Deserialize)]
pub struct GenerateLoadData {
    pub file_path: String,
    pub table_name: String,
    pub db_driver: String,
}