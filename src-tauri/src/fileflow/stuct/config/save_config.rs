use crate::fileflow::enums::database_engine::DatabaseEngine;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

/// Struct for saving database configuration details
#[derive(Deserialize, Serialize, Clone)]
pub struct SaveConfig {
    pub config_name: String,
    pub db_driver: DatabaseEngine,
    pub db_host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub db_name: String,
    pub sqlite_file_path: String,
}

impl SaveConfig {
    /// This function is used to get the size of a file.
    pub fn get_all_configs(config_file: &str) -> Vec<SaveConfig> {
        let path: PathBuf = PathBuf::from(config_file);
        let file: File = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };

        if file.metadata().unwrap().len() == 0 {
            return Vec::new();
        }

        let configs: Vec<SaveConfig> = serde_json::from_reader(file).unwrap_or(Vec::new());
        configs
    }

    /// This function is used to save a vector of SaveConfig to a json file.
    pub fn save_config(configs: &[SaveConfig], config_file: &str) -> io::Result<()> {
        let file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_file)
            .map_err(|e| format!("Failed to open file for writing: {e}"))
            .expect("Failed to open file for writing");
        serde_json::to_writer_pretty(file, &configs)
            .map_err(|e| format!("Failed to write to file: {e}"))
            .expect("Failed to write to file");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn get_test_config() -> SaveConfig {
        SaveConfig {
            config_name: "config_name".into(),
            db_driver: DatabaseEngine::SQLite,
            db_host: String::from("db_host"),
            port: String::from("port"),
            username: String::from("username"),
            password: String::from("password"),
            db_name: String::from("db_name"),
            sqlite_file_path: "sqlite_file_path".into(),
        }
    }

    #[tokio::test]
    async fn test_save_config_struct() {
        let config = get_test_config();

        assert_eq!(config.db_driver, DatabaseEngine::SQLite);
        assert_eq!(config.db_host, "db_host");
        assert_eq!(config.port, "port");
        assert_eq!(config.username, "username");
        assert_eq!(config.password, "password");
        assert_eq!(config.db_name, "db_name");
        assert_eq!(config.sqlite_file_path, "sqlite_file_path");
    }

    #[tokio::test]
    async fn test_save_config_serialization() {
        let config = get_test_config();

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: SaveConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.db_driver, DatabaseEngine::SQLite);
        assert_eq!(deserialized.config_name, "config_name");
        assert_eq!(deserialized.db_host, "db_host");
        assert_eq!(deserialized.port, "port");
        assert_eq!(deserialized.username, "username");
        assert_eq!(deserialized.password, "password");
        assert_eq!(deserialized.db_name, "db_name");
        assert_eq!(deserialized.sqlite_file_path, "sqlite_file_path");
    }

    #[tokio::test]
    async fn test_get_all_saved_configs_empty() {
        let configs = SaveConfig::get_all_configs("non_existent_file.json");
        assert!(configs.is_empty());
    }

    #[tokio::test]
    async fn test_get_all_saved_configs() {
        let config = get_test_config();
        let configs = vec![config.clone()];
        let config_file = "test_configs.json";

        // Save the config
        SaveConfig::save_config(&configs, config_file).unwrap();

        // Retrieve the config
        let loaded_configs = SaveConfig::get_all_configs(config_file);
        assert_eq!(loaded_configs.len(), 1);
        assert_eq!(loaded_configs[0].config_name, "config_name");
        assert_eq!(loaded_configs[0].db_driver, DatabaseEngine::SQLite);
        assert_eq!(loaded_configs[0].db_host, "db_host");
        assert_eq!(loaded_configs[0].port, "port");
        assert_eq!(loaded_configs[0].username, "username");
        assert_eq!(loaded_configs[0].password, "password");
        assert_eq!(loaded_configs[0].db_name, "db_name");
        assert_eq!(loaded_configs[0].sqlite_file_path, "sqlite_file_path");

        // Clean up
        std::fs::remove_file(config_file).unwrap();
    }

    #[tokio::test]
    async fn test_save_config() {
        let config = get_test_config();
        let configs = vec![config.clone()];
        let config_file = "test_save_configs.json";

        // Save the config
        SaveConfig::save_config(&configs, config_file).unwrap();

        // Retrieve the config
        let loaded_configs = SaveConfig::get_all_configs(config_file);
        assert_eq!(loaded_configs.len(), 1);
        assert_eq!(loaded_configs[0].config_name, "config_name");
        assert_eq!(loaded_configs[0].db_driver, DatabaseEngine::SQLite);
        assert_eq!(loaded_configs[0].db_host, "db_host");
        assert_eq!(loaded_configs[0].port, "port");
        assert_eq!(loaded_configs[0].username, "username");
        assert_eq!(loaded_configs[0].password, "password");
        assert_eq!(loaded_configs[0].db_name, "db_name");
        assert_eq!(loaded_configs[0].sqlite_file_path, "sqlite_file_path");

        // Clean up
        std::fs::remove_file(config_file).unwrap();
    }
}
