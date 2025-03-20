use serde::Deserialize;
use crate::fileflow::enumeration::database_engine::DatabaseEngine;

#[derive(Deserialize)]
pub struct InsertConfig {
    pub file_path: String,
    pub table_name: String,
    pub mode: String,
    pub db_driver: DatabaseEngine,
}
