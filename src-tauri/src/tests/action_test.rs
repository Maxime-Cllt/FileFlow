use crate::fileflow::database::connection::Connection;
use crate::fileflow::fast_insert::fast_insert;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::utils::constants::SQLITE;
use crate::tests::utils::{
    create_test_db, generate_csv_file, get_test_sqlite_config, remove_csv_file, remove_test_db,
};
use csv::{Reader, ReaderBuilder};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Pool, Row, Sqlite};
use std::fs::File;

#[tokio::test]
async fn test_fast_insert() {
    let sqlite_file_path: String = create_test_db("fast_insert".to_string());

    let config: DbConfig = get_test_sqlite_config(sqlite_file_path.to_string());
    let conn = Connection::connect(&config).await;

    assert!(conn.is_ok(), "Failed to connect to the database");

    let conn: Connection = conn.unwrap();

    let csv_file_path: String = generate_csv_file("test_fast_insert".to_string()).unwrap();

    let snake_case_headers: Vec<String> = vec!["header1".to_string(), "header2".to_string()];
    let final_table_name: &str = "test_table";

    let mut reader: Reader<File> = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(&csv_file_path).expect("Failed to open CSV file"));

    let result: Result<u64, String> = fast_insert(
        &conn,
        &mut reader,
        &snake_case_headers,
        final_table_name,
        SQLITE,
    )
    .await;

    // Ensure the number of rows inserted is as expected
    let inserted_count: u64 = result.unwrap();
    assert_eq!(inserted_count, 2, "Unexpected number of rows inserted");

    let pool: Pool<Sqlite> = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(sqlite_file_path.as_str())
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

    let _ = remove_test_db("fast_insert".to_string());
    let _ = remove_csv_file("test_fast_insert".to_string());
}
