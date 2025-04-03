use crate::fileflow::database::connection::{Connection, QueryResult};
use crate::fileflow::database::sql_builder::{
    build_create_table_sql, build_create_with_fixed_size_sql, build_drop_statement_sql,
    build_prepared_statement_sql, build_query_all_tables,
};
use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use crate::fileflow::enumeration::separator::SeparatorType;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::download_config::DownloadConfig;
use crate::tests::utils_tests::{
    create_test_db, get_test_maridb_config, get_test_mysql_config, get_test_pg_config,
    get_test_sqlite_config, remove_test_db,
};
use sqlx::testing::TestTermination;
use sqlx::{Error, Row};
use std::collections::HashMap;
use std::path::PathBuf;

#[tokio::test]
async fn test_get_connection_url() {
    let pg_config: DbConfig = get_test_pg_config();
    let mariadb_config: DbConfig = get_test_maridb_config();
    let mysql_config: DbConfig = get_test_mysql_config();
    let sqlite_config: DbConfig = get_test_sqlite_config(String::new());

    assert_eq!(
        Connection::get_connection_url(&pg_config).unwrap(),
        "postgres://postgres:password@localhost:5432/test_db"
    );
    assert_eq!(
        Connection::get_connection_url(&mariadb_config).unwrap(),
        "mysql://root:password@localhost:3306/test_db"
    );
    assert_eq!(
        Connection::get_connection_url(&mysql_config).unwrap(),
        "mysql://root:password@localhost:3306/test_db"
    );
    assert_eq!(
        Connection::get_connection_url(&sqlite_config).unwrap(),
        "test_db.db"
    );
}

#[tokio::test]
async fn test_sqlite_connection() {
    let file_path: String = create_test_db("sqlite_connection");
    let config: DbConfig = get_test_sqlite_config(file_path);
    let conn = Connection::connect(&config).await;
    assert!(conn.is_success(), "Failed to connect to the database");
    assert!(conn.is_ok());
    drop(conn);
    remove_test_db("sqlite_connection").expect("Failed to remove test table");
}



#[tokio::test]
async fn test_query_many_with_result() {
    let file_path: String = create_test_db("test_query_many_with_result");
    let config: DbConfig = get_test_sqlite_config(file_path);
    let conn: Result<Connection, Error> = Connection::connect(&config).await;

    assert!(conn.is_success(), "Failed to connect to the database");
    assert!(conn.is_ok());

    let conn: Connection = conn.unwrap();

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

    let query_result: Result<QueryResult, Error> = conn
        .query_many_with_result("SELECT * FROM test_table")
        .await;
    assert!(query_result.is_ok());

    drop(conn);

    let query_result: QueryResult = query_result.unwrap();

    if let QueryResult::SQLite(rows) = query_result {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), 2);
    }

    remove_test_db("test_query_many_with_result").expect("Failed to remove test query");
}

#[tokio::test]
async fn test_build_query_all_tables() {
    let test_cases: Vec<(&DatabaseEngine, String)> = vec![
        (
            &DatabaseEngine::MySQL,
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'test';".into(),
        ),
        (
            &DatabaseEngine::MariaDB,
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'test';".into(),
        ),
        (
            &DatabaseEngine::Postgres,
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'public';"
                .into(),
        ),
        (
            &DatabaseEngine::SQLite,
            "SELECT name FROM sqlite_master WHERE type='table';".into(),
        ),
    ];

    for (driver, expected) in test_cases {
        assert_eq!(
            build_query_all_tables(driver, "test"),
            expected,
            "Failed for driver: {:?}",
            driver
        );
    }
}

#[tokio::test]
async fn test_download_table() {
    let file_path: String = create_test_db("test_download_table");
    let config: DbConfig = get_test_sqlite_config(file_path);
    let conn: Result<Connection, Error> = Connection::connect(&config).await;

    assert!(conn.is_success(), "Failed to connect to the database");
    assert!(conn.is_ok());

    let conn: Connection = conn.unwrap();

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

    let query_result: QueryResult = conn
        .query_many_with_result("SELECT * FROM test_table")
        .await
        .unwrap();

    if let QueryResult::SQLite(rows) = query_result {
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

    export_table(&conn, &download_config, &download_config.table_name_list[0])
        .await
        .expect("Failed to export table");

    // check if file exists
    assert!(std::path::Path::new(&file_path).exists());

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

    export_table(&conn, &download_config, &download_config.table_name_list[0])
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
    drop(conn);
    std::fs::remove_file(&file_path).expect("Failed to remove file");
    remove_test_db("test_download_table").expect("Failed to remove test table");
}
