use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct SaveConfig {
    pub config_name: String,
    pub db_driver: String,
    pub db_host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub db_name: String,
    pub sqlite_file_path: String,
}
