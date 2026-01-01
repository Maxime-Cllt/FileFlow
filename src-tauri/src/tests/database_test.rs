use crate::tests::utils_tests::{
    create_test_db, get_test_maridb_config, get_test_mysql_config, get_test_pg_config,
    get_test_sqlite_config, remove_test_db,
};
use fileflow::fileflow::enums::query_rows::QueryRows;
use fileflow::fileflow::enums::separator_type::SeparatorType;
use fileflow::fileflow::stuct::config::db_config::DatabaseConfig;
use fileflow::fileflow::stuct::config::download_config::DownloadConfig;
use fileflow::fileflow::stuct::database::database_connection::DatabaseConnection;
use fileflow::fileflow::stuct::io::table_exporter::TableExporter;
use sqlx::testing::TestTermination;
use sqlx::{Error, Row};
use std::path::PathBuf;

#[tokio::test]
async fn test_get_connection_url() {
    assert_eq!(
        DatabaseConnection::get_connection_url(&get_test_pg_config()),
        "postgres://postgres:password@localhost:5432/test_db"
    );
    assert_eq!(
        DatabaseConnection::get_connection_url(&get_test_maridb_config()),
        "mysql://root:password@localhost:3306/test_db"
    );
    assert_eq!(
        DatabaseConnection::get_connection_url(&get_test_mysql_config()),
        "mysql://root:password@localhost:3306/test_db"
    );
    assert_eq!(
        DatabaseConnection::get_connection_url(&get_test_sqlite_config(String::new())),
        "test_db.db"
    );
}

#[tokio::test]
async fn test_sqlite_connection() {
    let file_path: String = create_test_db("sqlite_connection");
    let config: DatabaseConfig = get_test_sqlite_config(file_path);
    let conn = DatabaseConnection::connect(&config).await;

    assert!(conn.is_success(), "Failed to connect to the database");
    assert!(conn.is_ok());

    let conn: DatabaseConnection = conn.unwrap();
    conn.disconnect().await;
    remove_test_db("sqlite_connection").expect("Failed to remove test table");
}

#[tokio::test]
async fn test_query_many_with_result() {
    let file_path: String = create_test_db("test_query_many_with_result");
    let config: DatabaseConfig = get_test_sqlite_config(file_path);
    let conn: Result<DatabaseConnection, Error> = DatabaseConnection::connect(&config).await;

    assert!(conn.is_success(), "Failed to connect to the database");
    assert!(conn.is_ok());

    let conn: DatabaseConnection = conn.unwrap();

    const SQL_ARRAY: [&str; 3] = [
        "DROP TABLE IF EXISTS test_table",
        "CREATE TABLE test_table (header1 VARCHAR(10), header2 VARCHAR(10))",
        "INSERT INTO test_table (header1, header2) VALUES ('value1', 'value2')",
    ];

    for sql in SQL_ARRAY.iter() {
        if let Err(e) = conn.query(sql).await {
            println!("Error: {:?} for query: {sql}", e);
        }
    }

    let query_result: Result<QueryRows, Error> = conn
        .query_many_with_result("SELECT * FROM test_table")
        .await;
    assert!(query_result.is_ok());

    let query_result: QueryRows = query_result.unwrap();

    if let QueryRows::SQLite(rows) = query_result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), 2);
    }

    conn.disconnect().await;
    remove_test_db("test_query_many_with_result").expect("Failed to remove test query");
}

#[tokio::test]
async fn test_download_table() {
    let file_path: String = create_test_db("test_download_table");
    let config: DatabaseConfig = get_test_sqlite_config(file_path);
    let conn: Result<DatabaseConnection, Error> = DatabaseConnection::connect(&config).await;

    assert!(conn.is_success(), "Failed to connect to the database");
    assert!(conn.is_ok());

    let conn: DatabaseConnection = conn.unwrap();

    const SQL_ARRAY: [&str; 4] = [
        "DROP TABLE IF EXISTS test_table",
        "CREATE TABLE test_table (header1 VARCHAR(10), header2 VARCHAR(10))",
        "INSERT INTO test_table (header1, header2) VALUES ('value1', 'value2')",
        "INSERT INTO test_table (header1, header2) VALUES ('value3', 'value4')",
    ];

    for sql in SQL_ARRAY.iter() {
        if let Err(e) = conn.query(sql).await {
            println!("Error: {:?} for query: {sql}", e);
        }
    }

    let query_result: QueryRows = conn
        .query_many_with_result("SELECT * FROM test_table")
        .await
        .unwrap();

    if let QueryRows::SQLite(rows) = query_result {
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 2);
    }

    let download_config: DownloadConfig = DownloadConfig {
        separator: SeparatorType::Semicolon,
        table_name_list: vec!["test_table".into()],
        location: "./".into(),
    };

    let file_path: PathBuf = PathBuf::from(format!(
        "{}/{}_export.csv",
        download_config.location, download_config.table_name_list[0]
    ));

    TableExporter::export_table(&conn, &download_config, &download_config.table_name_list[0])
        .await
        .expect("Failed to export table");

    assert!(std::path::Path::new(&file_path).exists()); // check if file exists

    // get the content of the file
    let content: String = std::fs::read_to_string(&file_path).expect("Failed to read file");
    assert_eq!(
        content, "header1;header2\nvalue1;value2\nvalue3;value4\n",
        "Failed to export table"
    );

    let download_config: DownloadConfig = DownloadConfig {
        separator: SeparatorType::Comma,
        table_name_list: vec!["test_table".into()],
        location: "./".into(),
    };

    TableExporter::export_table(&conn, &download_config, &download_config.table_name_list[0])
        .await
        .expect("Failed to export table");

    // check if file exists
    assert!(std::path::Path::new(&file_path).exists());

    // get the content of the file
    let content: String = std::fs::read_to_string(&file_path).expect("Failed to read file");
    assert_eq!(
        content, "header1,header2\nvalue1,value2\nvalue3,value4\n",
        "Failed to export table"
    );

    // Clean up
    std::fs::remove_file(&file_path).expect("Failed to remove file");
    remove_test_db("test_download_table").expect("Failed to remove test table");
}
