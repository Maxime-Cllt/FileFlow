use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::load_data_struct::GenerateLoadData;
use crate::fileflow::stuct::save_config::SaveConfig;

#[tokio::test]
async fn test_db_config() {
    let config = DbConfig {
        db_driver: "db_driver".to_string(),
        username: "username".to_string(),
        password: "password".to_string(),
        db_host: "db_host".to_string(),
        port: "port".to_string(),
        db_name: "db_name".to_string(),
        sqlite_file_path: "sqlite_file_path".to_string(),
    };

    assert_eq!(config.db_driver, "db_driver");
    assert_eq!(config.username, "username");
    assert_eq!(config.password, "password");
    assert_eq!(config.db_host, "db_host");
    assert_eq!(config.port, "port");
    assert_eq!(config.db_name, "db_name");
    assert_eq!(config.sqlite_file_path, "sqlite_file_path");
}

#[tokio::test]
async fn test_insert_config() {
    let config = InsertConfig {
        file_path: "file_path".to_string(),
        table_name: "table_name".to_string(),
        mode: "mode".to_string(),
        db_driver: "db_driver".to_string(),
    };

    assert_eq!(config.db_driver, "db_driver");
    assert_eq!(config.file_path, "file_path");
    assert_eq!(config.table_name, "table_name");
    assert_eq!(config.mode, "mode");
}

#[tokio::test]
async fn test_save_config() {
    let config = SaveConfig {
        config_name: "config_name".to_string(),
        db_driver: "db_driver".to_string(),
        db_host: "db_host".to_string(),
        port: "port".to_string(),
        username: "username".to_string(),
        password: "password".to_string(),
        db_name: "db_name".to_string(),
        sqlite_file_path: "sqlite_file_path".to_string(),
    };

    assert_eq!(config.db_driver, "db_driver");
    assert_eq!(config.db_host, "db_host");
    assert_eq!(config.port, "port");
    assert_eq!(config.username, "username");
    assert_eq!(config.password, "password");
    assert_eq!(config.db_name, "db_name");
    assert_eq!(config.sqlite_file_path, "sqlite_file_path");
}

#[tokio::test]
async fn test_load_data_struct() {
    let config: GenerateLoadData = GenerateLoadData {
        file_path: "file_path".to_string(),
        table_name: "table_name".to_string(),
        db_driver: "db_driver".to_string(),
    };

    assert_eq!(config.db_driver, "db_driver");
    assert_eq!(config.file_path, "file_path");
    assert_eq!(config.table_name, "table_name");
}
