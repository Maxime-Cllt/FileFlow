use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use crate::fileflow::enumeration::insertion_type::InsertionType;
use crate::fileflow::enumeration::separator::SeparatorType;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::download_config::DownloadConfig;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::save_config::SaveConfig;

#[tokio::test]
async fn test_db_config() {
    let config = DbConfig {
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
async fn test_insert_config() {
    let config = InsertConfig {
        files_path: vec!["file_path".into()],
        table_name: "table_name".into(),
        mode: InsertionType::Fast,
        db_driver: DatabaseEngine::Postgres,
    };

    assert_eq!(config.db_driver, DatabaseEngine::Postgres);
    assert_eq!(config.files_path, vec!["file_path"]);
    assert_eq!(config.table_name, "table_name");
    assert_eq!(config.mode, InsertionType::Fast);
}

#[tokio::test]
async fn test_save_config() {
    let config = SaveConfig {
        config_name: "config_name".into(),
        db_driver: DatabaseEngine::Postgres,
        db_host: String::from("db_host"),
        port: String::from("port"),
        username: String::from("username"),
        password: String::from("password"),
        db_name: String::from("db_name"),
        sqlite_file_path: "sqlite_file_path".into(),
    };

    assert_eq!(config.db_driver, DatabaseEngine::Postgres);
    assert_eq!(config.db_host, "db_host");
    assert_eq!(config.port, "port");
    assert_eq!(config.username, "username");
    assert_eq!(config.password, "password");
    assert_eq!(config.db_name, "db_name");
    assert_eq!(config.sqlite_file_path, "sqlite_file_path");
}

#[tokio::test]
async fn test_downlopad_config() {
    let config = DownloadConfig {
        table_name_list: vec!["table1".into(), "table2".into()],
        location: "location".into(),
        separator: SeparatorType::Comma,
    };

    assert_eq!(config.table_name_list.len(), 2);
    assert_eq!(config.table_name_list[0], "table1");
    assert_eq!(config.table_name_list[1], "table2");
    assert_eq!(config.location, "location");
    assert_eq!(config.separator, SeparatorType::Comma);
}
