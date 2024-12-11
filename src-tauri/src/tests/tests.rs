// unit tests
#[cfg(test)]
mod tests {
    use crate::fileflow::action::actions::{generate_load_data_sql, load_database_config};
    use crate::fileflow::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
    use crate::fileflow::database::connection::DatabaseConnection;
    use crate::fileflow::fast_insert::fast_insert;
    use crate::fileflow::fileflowlib::{
        detect_separator_in_file, get_create_statement, get_create_statement_with_fixed_size,
        get_drop_statement, get_formated_column_names, get_insert_into_statement,
    };
    use crate::fileflow::stuct::db_config::DbConfig;
    use crate::fileflow::stuct::insert_config::InsertConfig;
    use crate::fileflow::stuct::load_data_struct::GenerateLoadData;
    use crate::fileflow::stuct::save_config::SaveConfig;
    use crate::tests::utils::{
        create_test_db, generate_csv_file, get_test_maridb_config, get_test_mysql_config,
        get_test_pg_config, get_test_sqlite_config, remove_csv_file, remove_test_db,
    };
    use csv::{Reader, ReaderBuilder};
    use sqlx::sqlite::SqliteRow;
    use sqlx::testing::TestTermination;
    use sqlx::{Error, Pool, Row, Sqlite};
    use std::collections::HashMap;
    use std::fs::File;

    #[tokio::test]
    async fn test_get_connection_url() {
        let pg_config: DbConfig = get_test_pg_config();
        let mariadb_config: DbConfig = get_test_maridb_config();
        let mysql_config: DbConfig = get_test_mysql_config();
        let sqlite_config: DbConfig = get_test_sqlite_config("".to_string());

        assert_eq!(
            DatabaseConnection::get_connection_url(&pg_config).unwrap(),
            "postgres://postgres:password@localhost:5432/test_db"
        );
        assert_eq!(
            DatabaseConnection::get_connection_url(&mariadb_config).unwrap(),
            "mysql://root:password@localhost:3306/test_db"
        );
        assert_eq!(
            DatabaseConnection::get_connection_url(&mysql_config).unwrap(),
            "mysql://root:password@localhost:3306/test_db"
        );
        assert_eq!(
            DatabaseConnection::get_connection_url(&sqlite_config).unwrap(),
            "test_db.db"
        );
    }

    #[tokio::test]
    async fn test_db_connection() {
        let config: String = load_database_config().await.unwrap();
        let config: DbConfig = serde_json::from_str(&config).unwrap();
        let conn = DatabaseConnection::connect(&config).await;
        assert_eq!(conn.is_ok(), true);
        assert_eq!(conn.is_err(), false);
    }

    #[tokio::test]
    async fn test_sqlite_connection() {
        let file_path: String = create_test_db("sqlite_connection".to_string());
        let config: DbConfig = get_test_sqlite_config(file_path);
        let conn = DatabaseConnection::connect(&config).await;
        assert!(conn.is_success(), "Failed to connect to the database");
        assert_eq!(conn.is_ok(), true);
        assert_eq!(conn.is_err(), false);
        let _ = remove_test_db("sqlite_connection".to_string());
    }

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
        let snake_case_headers: Vec<String> = vec!["header1".to_string(), "header2".to_string()];
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

        let snake_case_headers: Vec<String> = vec!["header1".to_string()];
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
        let snake_case_headers: Vec<String> = vec!["header1".to_string(), "header2".to_string()];
        let map_max_length: HashMap<&str, usize> =
            snake_case_headers.iter().map(|h| (h.as_str(), 0)).collect();
        let final_table_name: &str = "test_table";

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

        let final_columns: Vec<String> = vec!["header1".to_string(), "header2".to_string()];

        for (driver, expected) in db_driver {
            let result: Result<String, String> = get_create_statement_with_fixed_size(
                driver,
                final_table_name,
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
                final_table_name,
                &map_max_length,
                &final_columns,
            );
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[tokio::test]
    async fn test_get_formated_column_names() {
        let headers: Vec<String> = vec!["header 1".to_string(), " header2".to_string()];
        let formatted_headers: Vec<String> = get_formated_column_names(headers);
        assert_eq!(
            formatted_headers,
            vec!["header_1".to_string(), "_header2".to_string()]
        );

        let headers: Vec<String> = vec![
            "header    1".to_string(),
            "".to_string(),
            "header2".to_string(),
        ];
        let formatted_headers: Vec<String> = get_formated_column_names(headers);
        assert_eq!(
            formatted_headers,
            vec![
                "header____1".to_string(),
                "column_2".to_string(),
                "header2".to_string()
            ]
        );
    }

    #[tokio::test]
    async fn test_generate_load_data_sql() {
        let csv_file_path: String =
            generate_csv_file("test_generate_load_data_sql".to_string()).unwrap();

        let snake_case_headers: Vec<String> = vec!["header1".to_string(), "header2".to_string()];
        let final_table_name: &str = "test_table";

        let config = GenerateLoadData {
            file_path: csv_file_path.clone(),
            db_driver: MARIADB.to_string(),
            table_name: "test_table".to_string(),
        };

        let mut size_map: HashMap<&str, usize> = HashMap::new();
        for header in &snake_case_headers {
            size_map.insert(header.as_str(), 7);
        }

        let separator: char = detect_separator_in_file(&csv_file_path).unwrap();

        let mut sql: String = String::new();

        sql.push_str(
            get_drop_statement(MARIADB, final_table_name)
                .unwrap()
                .as_str(),
        );
        sql.push_str(";");
        sql.push_str("\n\n");

        sql.push_str(
            get_create_statement_with_fixed_size(
                MARIADB,
                final_table_name,
                &size_map,
                &snake_case_headers,
            )
            .unwrap()
            .as_str(),
        );
        sql.push_str("\n\n");

        sql.push_str("LOAD DATA INFILE '");
        sql.push_str(csv_file_path.as_str());
        sql.push_str("'\nINTO TABLE ");
        sql.push_str(final_table_name);
        sql.push_str("\nCHARACTER SET utf8\n");
        sql.push_str("FIELDS TERMINATED BY '");
        sql.push(separator);
        sql.push_str("'\n");
        sql.push_str("ENCLOSED BY '\"'\nLINES TERMINATED BY '\\n'\nIGNORE 1 ROWS (");
        sql.push_str("header1, header2");
        sql.push_str(");");

        let result: Result<String, String> = generate_load_data_sql(config).await;
        let result: String = result.unwrap();

        assert_eq!(result, sql);
        let _ = remove_csv_file("test_generate_load_data_sql".to_string());
    }

    #[tokio::test]
    async fn test_fast_insert() {
        let sqlite_file_path: String = create_test_db("fast_insert".to_string());

        let config: DbConfig = get_test_sqlite_config(sqlite_file_path.to_string());
        let conn = DatabaseConnection::connect(&config).await;

        assert!(conn.is_ok(), "Failed to connect to the database");

        let conn: DatabaseConnection = conn.unwrap();

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

    #[tokio::test]
    async fn test_detect_separator_in_file() {
        let csv_file_path: String =
            generate_csv_file("test_detect_separator_in_file".to_string()).unwrap();
        let separator: char = detect_separator_in_file(&csv_file_path).unwrap();
        assert_eq!(separator, ',');
        let _ = remove_csv_file("test_detect_separator_in_file".to_string());
    }
}
