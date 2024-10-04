use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]  // Derive both Deserialize and Serialize
pub struct SaveConfig {
    pub db_driver: String,
    pub db_host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub db_name: String,
}
