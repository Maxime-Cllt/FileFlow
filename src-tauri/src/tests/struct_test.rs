use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::save_config::SaveConfig;

#[tokio::test]
async fn test_db_config() {
    let config = DbConfig {
        db_driver: String::from("db_driver"),
        username: String::from("username"),
        password: String::from("password"),
        db_host: String::from("db_host"),
        port: String::from("port"),
        db_name: String::from("db_name"),
        sqlite_file_path: "sqlite_file_path".into(),
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
        file_path: "file_path".into(),
        table_name: "table_name".into(),
        mode: "mode".into(),
        db_driver: String::from("db_driver"),
    };

    assert_eq!(config.db_driver, "db_driver");
    assert_eq!(config.file_path, "file_path");
    assert_eq!(config.table_name, "table_name");
    assert_eq!(config.mode, "mode");
}

#[tokio::test]
async fn test_save_config() {
    let config = SaveConfig {
        config_name: "config_name".into(),
        db_driver: String::from("db_driver"),
        db_host: String::from("db_host"),
        port: String::from("port"),
        username: String::from("username"),
        password: String::from("password"),
        db_name: String::from("db_name"),
        sqlite_file_path: "sqlite_file_path".into(),
    };

    assert_eq!(config.db_driver, "db_driver");
    assert_eq!(config.db_host, "db_host");
    assert_eq!(config.port, "port");
    assert_eq!(config.username, "username");
    assert_eq!(config.password, "password");
    assert_eq!(config.db_name, "db_name");
    assert_eq!(config.sqlite_file_path, "sqlite_file_path");
}
