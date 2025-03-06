use crate::fileflow::database::connection::Connection;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::load_data_struct::GenerateLoadData;
use crate::fileflow::utils::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use crate::fileflow::utils::fileflowlib::{build_load_data, get_formated_column_names};
use crate::fileflow::utils::sql::{
    get_create_statement, get_create_statement_with_fixed_size, get_drop_statement,
    get_insert_into_statement,
};
use crate::tests::utils::{
    create_test_db, generate_csv_file, get_test_maridb_config, get_test_mysql_config,
    get_test_pg_config, get_test_sqlite_config, remove_test_db,
};
use sqlx::testing::TestTermination;
use std::collections::HashMap;

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
    let file_path: String = create_test_db(String::from("sqlite_connection"));
    let config: DbConfig = get_test_sqlite_config(file_path);
    let conn = Connection::connect(&config).await;
    assert!(conn.is_success(), "Failed to connect to the database");
    assert!(conn.is_ok());
    let _ = remove_test_db(String::from("sqlite_connection"));
}

#[tokio::test]
async fn test_get_drop_statement() {
    assert_eq!(
        get_drop_statement(SQLITE, "table_name").unwrap(),
        "DROP TABLE IF EXISTS \"table_name\""
    );
    assert_eq!(
        get_drop_statement(MYSQL, "table_name").unwrap(),
        "DROP TABLE IF EXISTS `table_name`"
    );
    assert_eq!(
        get_drop_statement(POSTGRES, "table_name").unwrap(),
        "DROP TABLE IF EXISTS \"table_name\""
    );
    assert_eq!(
        get_drop_statement("unsupported", "table_name").unwrap_err(),
        "Unsupported database driver"
    );

    assert_eq!(
        get_drop_statement(SQLITE, "").unwrap(),
        "DROP TABLE IF EXISTS \"\""
    );
    assert_eq!(
        get_drop_statement(MYSQL, "").unwrap(),
        "DROP TABLE IF EXISTS ``"
    );
    assert_eq!(
        get_drop_statement(POSTGRES, "").unwrap(),
        "DROP TABLE IF EXISTS \"\""
    );
}

#[tokio::test]
async fn test_get_insert_into_statement() {
    assert_eq!(
        get_insert_into_statement(SQLITE, "table_name", "columns").unwrap(),
        "INSERT INTO \"table_name\" (columns) VALUES "
    );
    assert_eq!(
        get_insert_into_statement(MYSQL, "table_name", "columns").unwrap(),
        "INSERT INTO `table_name` (columns) VALUES "
    );
    assert_eq!(
        get_insert_into_statement(POSTGRES, "table_name", "columns").unwrap(),
        "INSERT INTO \"table_name\" (columns) VALUES "
    );
    assert_eq!(
        get_insert_into_statement("unsupported", "table_name", "columns").unwrap_err(),
        "Unsupported database driver"
    );

    assert_eq!(
        get_insert_into_statement(SQLITE, "table_name", "").unwrap(),
        "INSERT INTO \"table_name\" () VALUES "
    );
    assert_eq!(
        get_insert_into_statement(MYSQL, "table_name", "").unwrap(),
        "INSERT INTO `table_name` () VALUES "
    );
    assert_eq!(
        get_insert_into_statement(POSTGRES, "table_name", "").unwrap(),
        "INSERT INTO \"table_name\" () VALUES "
    );

    assert_eq!(
        get_insert_into_statement(SQLITE, "table_name", "header1, header2").unwrap(),
        "INSERT INTO \"table_name\" (header1, header2) VALUES "
    );
    assert_eq!(
        get_insert_into_statement(MYSQL, "table_name", "header1, header2").unwrap(),
        "INSERT INTO `table_name` (header1, header2) VALUES "
    );
    assert_eq!(
        get_insert_into_statement(POSTGRES, "table_name", "header1, header2").unwrap(),
        "INSERT INTO \"table_name\" (header1, header2) VALUES "
    );
}

#[tokio::test]
async fn test_get_create_statement() {
    let snake_case_headers: Vec<String> = vec!["header1".into(), "header2".into()];
    assert_eq!(
        get_create_statement(SQLITE, "table_name", &snake_case_headers).unwrap(),
        "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)"
    );
    assert_eq!(
        get_create_statement(MYSQL, "table_name", &snake_case_headers).unwrap(),
        "CREATE TABLE `table_name` (header1 TEXT, header2 TEXT)"
    );
    assert_eq!(
        get_create_statement(POSTGRES, "table_name", &snake_case_headers).unwrap(),
        "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)"
    );
    assert_eq!(
        get_create_statement("unsupported", "table_name", &snake_case_headers).unwrap_err(),
        "Unsupported database driver"
    );

    let snake_case_headers: Vec<String> = vec!["header1".into()];
    assert_eq!(
        get_create_statement(SQLITE, "table_name", &snake_case_headers).unwrap(),
        "CREATE TABLE \"table_name\" (header1 TEXT)"
    );
    assert_eq!(
        get_create_statement(MYSQL, "table_name", &snake_case_headers).unwrap(),
        "CREATE TABLE `table_name` (header1 TEXT)"
    );
    assert_eq!(
        get_create_statement(POSTGRES, "table_name", &snake_case_headers).unwrap(),
        "CREATE TABLE \"table_name\" (header1 TEXT)"
    );
}

#[tokio::test]
async fn test_get_create_statement_with_fixed_size() {
    const FINAL_TABLE_NAME: &str = "test_table";

    let snake_case_headers: Vec<String> = vec!["header1".into(), "header2".into()];
    let map_max_length: HashMap<&str, usize> =
        snake_case_headers.iter().map(|h| (h.as_str(), 0)).collect();

    let mut db_driver: HashMap<&str, &str> = HashMap::new();
    db_driver.insert(
        POSTGRES,
        "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(0), \"header2\" VARCHAR(0));",
    );
    db_driver.insert(
        MYSQL,
        "CREATE TABLE `test_table` (`header1` VARCHAR(0), `header2` VARCHAR(0));",
    );
    db_driver.insert(
        MARIADB,
        "CREATE TABLE `test_table` (`header1` VARCHAR(0), `header2` VARCHAR(0));",
    );
    db_driver.insert(
        SQLITE,
        "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(0), \"header2\" VARCHAR(0));",
    );

    let final_columns: Vec<String> = vec!["header1".into(), "header2".into()];

    for (driver, expected) in db_driver {
        let result: Result<String, String> = get_create_statement_with_fixed_size(
            driver,
            FINAL_TABLE_NAME,
            &map_max_length,
            &final_columns,
        );
        assert_eq!(result.unwrap(), expected);
    }

    let map_max_length: HashMap<&str, usize> = snake_case_headers
        .iter()
        .map(|h| (h.as_str(), 10))
        .collect();
    let mut db_driver: HashMap<&str, &str> = HashMap::new();
    db_driver.insert(
        POSTGRES,
        "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(10), \"header2\" VARCHAR(10));",
    );
    db_driver.insert(
        MYSQL,
        "CREATE TABLE `test_table` (`header1` VARCHAR(10), `header2` VARCHAR(10));",
    );
    db_driver.insert(
        MARIADB,
        "CREATE TABLE `test_table` (`header1` VARCHAR(10), `header2` VARCHAR(10));",
    );
    db_driver.insert(
        SQLITE,
        "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(10), \"header2\" VARCHAR(10));",
    );

    for (driver, expected) in db_driver {
        let result: Result<String, String> = get_create_statement_with_fixed_size(
            driver,
            FINAL_TABLE_NAME,
            &map_max_length,
            &final_columns,
        );
        assert_eq!(result.unwrap(), expected);
    }
}

#[tokio::test]
async fn test_get_formated_column_names() {
    let headers: Vec<String> = vec!["header 1".into(), " header2".into()];
    let formatted_headers: Vec<String> = get_formated_column_names(headers);
    assert_eq!(formatted_headers, vec!["header_1", "_header2"]);

    let headers: Vec<String> = vec!["header    1".into(), String::new(), "header2".into()];
    let formatted_headers: Vec<String> = get_formated_column_names(headers);
    assert_eq!(
        formatted_headers,
        vec!["header____1", "column_2", "header2"]
    );
}

#[tokio::test]
async fn test_build_load_data_mysql() {
    let file_path: String = generate_csv_file("test_build_load_data_mysql").unwrap();
    let config = GenerateLoadData {
        file_path,
        db_driver: MYSQL.into(),
        table_name: String::from("test_table"),
    };
    let separator: char = ',';
    let columns: Vec<String> = vec!["header1".into(), "header2".into()];

    let expected_sql = format!(
        "LOAD DATA INFILE '{}'\nINTO TABLE test_table\nCHARACTER SET utf8\nFIELDS TERMINATED BY '{}'\nENCLOSED BY '\"'\nLINES TERMINATED BY '\\n'\nIGNORE 1 ROWS (header1, header2);",
        config.file_path, separator
    );

    let result = build_load_data(config, separator, columns);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_sql);
}

#[tokio::test]
async fn test_build_load_data_postgres() {
    const SEPARATOR: char = ',';

    let file_path: String = generate_csv_file("test_build_load_data_postgres").unwrap();

    let config = GenerateLoadData {
        file_path,
        db_driver: String::from(POSTGRES),
        table_name: String::from("test_table"),
    };
    let columns: Vec<String> = vec!["header1".into(), "header2".into()];

    let expected_sql:String = format!(
        "COPY test_table (header1, header2)\nFROM '{}'\nWITH (FORMAT csv, HEADER true, DELIMITER '{}', QUOTE '\"');",
        config.file_path, SEPARATOR
    );

    let result = build_load_data(config, SEPARATOR, columns);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_sql);
}

#[tokio::test]
async fn test_build_load_data_invalid_driver() {
    const SEPARATOR: char = ',';

    let file_path: String = generate_csv_file("test_build_load_data_invalid_driver").unwrap();
    let config = GenerateLoadData {
        file_path,
        db_driver: "invalid_driver".into(),
        table_name: String::from("test_table"),
    };
    let columns: Vec<String> = vec!["header1".into(), "header2".into()];

    let result = build_load_data(config, SEPARATOR, columns);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Unsupported database driver for this operation"
    );
}
