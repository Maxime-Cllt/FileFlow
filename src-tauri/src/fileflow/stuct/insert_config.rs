use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use crate::fileflow::enumeration::insertion_type::InsertionType;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InsertConfig {
    pub files_path: Vec<String>,
    pub table_name: String,
    pub mode: InsertionType,
    pub db_driver: DatabaseEngine,
}
