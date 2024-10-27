use serde::Deserialize;


#[derive(Deserialize)]
pub struct InsertConfig {
    pub file_path: String,
    pub table_name: String,
    pub mode: String,
    pub db_driver: String,
}