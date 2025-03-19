use crate::fileflow::action::insertion_mode::fast_insert;
use crate::fileflow::database::connection::Connection;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::save_config::SaveConfig;
use crate::fileflow::utils::constants::SQLITE;
use crate::fileflow::utils::fileflowlib::{get_all_saved_configs, save_config};
use crate::tests::utils::{
    create_test_db, delete_config_file, generate_csv_file, get_test_save_config,
    get_test_sqlite_config, remove_csv_file, remove_test_db,
};
use csv::{Reader, ReaderBuilder};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Pool, Row, Sqlite};
use std::fs::File;

#[tokio::test]
async fn test_fast_insert() {
    let sqlite_file_path: String = create_test_db("fast_insert");

    let config: DbConfig = get_test_sqlite_config(sqlite_file_path.clone());
    let conn: Result<Connection, Error> = Connection::connect(&config).await;

    assert!(conn.is_ok(), "Failed to connect to the database");

    let conn: Connection = conn.unwrap();

    let csv_file_path: String =
        generate_csv_file("test_fast_insert").expect("Failed to generate csv file");

    let snake_case_headers: Vec<String> = vec!["header1".into(), "header2".into()];
    let final_table_name: &str = "test_table";

    let mut reader: Reader<File> = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(&csv_file_path).expect("Failed to open CSV file"));

    let result: Result<u32, String> = fast_insert(
        &conn,
        &mut reader,
        &snake_case_headers,
        final_table_name,
        SQLITE,
    )
    .await;

    // Ensure the number of rows inserted is as expected
    let inserted_count: u32 = result.expect("Failed to insert data");
    assert_eq!(inserted_count, 2, "Unexpected number of rows inserted");

    let pool: Pool<Sqlite> = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&sqlite_file_path)
        .await
        .expect("Failed to create a connection pool");

    // Test table existence
    let query: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'";
    let query_result: Result<SqliteRow, Error> = sqlx::query(query).fetch_one(&pool).await;

    let query_result: SqliteRow =
        query_result.expect("Failed to query the database for table existence");
    let value: String = query_result.get("name");

    assert_eq!(value, "test_table");

    // Test fetching a row
    let query: &str = "SELECT header1,header2 FROM test_table LIMIT 1";
    let row: SqliteRow = sqlx::query(query)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch a row");
    let value1: String = row.get("header1");
    let value2: String = row.get("header2");

    assert_eq!(value1, "value1");
    assert_eq!(value2, "value2");

    assert_ne!(value1, "value3");
    assert_ne!(value2, "value4");

    remove_test_db("fast_insert").expect("Failed to remove test table");
    remove_csv_file("test_fast_insert").expect("Failed to remove CSV file");
}

#[tokio::test]
async fn test_configs_serealization_deserialization() {
    const CONFIG_NAME: &str = "test_get_all_configs.json";

    let config1: SaveConfig = get_test_save_config("config1");
    let config2: SaveConfig = get_test_save_config("config2");
    let config3: SaveConfig = get_test_save_config("config3");
    let configs_list: Vec<SaveConfig> = vec![config1, config2, config3];

    save_config(&configs_list, CONFIG_NAME).expect("Failed to save configs");

    let deserialized_configs: Vec<SaveConfig> = get_all_saved_configs("test_get_all_configs.json");

    assert_eq!(3, deserialized_configs.len());

    assert_eq!(
        configs_list[0].config_name,
        deserialized_configs[0].config_name
    );
    assert_eq!(
        configs_list[1].config_name,
        deserialized_configs[1].config_name
    );
    assert_eq!(
        configs_list[2].config_name,
        deserialized_configs[2].config_name
    );

    delete_config_file(CONFIG_NAME).expect("Failed to delete config file");
}
