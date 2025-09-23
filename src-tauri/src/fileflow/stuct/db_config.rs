use crate::fileflow::enums::database_engine::DatabaseEngine;
use serde::{Deserialize, Serialize};

/// Database configuration structure
#[derive(Deserialize, Clone, Serialize)]
pub struct DatabaseConfig {
    pub db_driver: DatabaseEngine,
    pub db_host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub db_name: String,
    pub sqlite_file_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_config() {
        let config = DatabaseConfig {
            db_driver: DatabaseEngine::Postgres,
            username: String::from("username"),
            password: String::from("password"),
            db_host: String::from("db_host"),
            port: String::from("port"),
            db_name: String::from("db_name"),
            sqlite_file_path: "sqlite_file_path".into(),
        };

        assert_eq!(config.db_driver, DatabaseEngine::Postgres);
        assert_eq!(config.username, "username");
        assert_eq!(config.password, "password");
        assert_eq!(config.db_host, "db_host");
        assert_eq!(config.port, "port");
        assert_eq!(config.db_name, "db_name");
        assert_eq!(config.sqlite_file_path, "sqlite_file_path");
    }

    #[tokio::test]
    async fn test_db_config_serialization() {
        let config = DatabaseConfig {
            db_driver: DatabaseEngine::MySQL,
            username: String::from("username"),
            password: String::from("password"),
            db_host: String::from("db_host"),
            port: String::from("port"),
            db_name: String::from("db_name"),
            sqlite_file_path: "sqlite_file_path".into(),
        };
        let serialized = serde_json::to_string(&config).unwrap();
        let expected = r#"{"db_driver":"mysql","db_host":"db_host","port":"port","username":"username","password":"password","db_name":"db_name","sqlite_file_path":"sqlite_file_path"}"#;
        assert_eq!(serialized, expected);
        let deserialized: DatabaseConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.db_driver, DatabaseEngine::MySQL);
        assert_eq!(deserialized.username, "username");
        assert_eq!(deserialized.password, "password");
        assert_eq!(deserialized.db_host, "db_host");
        assert_eq!(deserialized.port, "port");
        assert_eq!(deserialized.db_name, "db_name");
        assert_eq!(deserialized.sqlite_file_path, "sqlite_file_path");
    }
}
