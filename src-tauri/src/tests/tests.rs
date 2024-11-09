// unit tests
#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::fileflow::database_connection::DatabaseConnection;
    use crate::fileflow::fileflow::{get_create_statement, get_drop_statement, get_insert_into_statement};
    use crate::fileflow::stuct::db_config::DbConfig;
    use crate::fileflow::stuct::insert_config::InsertConfig;
    use crate::fileflow::stuct::save_config::SaveConfig;
    use sqlx::testing::TestTermination;
    use std::path::PathBuf;
    use csv::{Reader, ReaderBuilder, Writer};
    use sqlx::{Error, Row};
    use sqlx::sqlite::SqliteRow;
    use crate::fileflow::action::actions::load_database_config;

    #[tokio::test]
    async fn test_get_connection_url() {
        let pg_config = DbConfig {
            db_driver: "postgres".to_string(),
            username: "postgres".to_string(),
            password: "password".to_string(),
            db_host: "localhost".to_string(),
            port: "5432".to_string(),
            db_name: "test_db".to_string(),
            sqlite_file_path: "".to_string(),
        };

        let mariadb_config = DbConfig {
            db_driver: "mariadb".to_string(),
            username: "root".to_string(),
            password: "password".to_string(),
            db_host: "localhost".to_string(),
            port: "3306".to_string(),
            db_name: "test_db".to_string(),
            sqlite_file_path: "".to_string(),
        };

        let mysql_config = DbConfig {
            db_driver: "mysql".to_string(),
            username: "root".to_string(),
            password: "password".to_string(),
            db_host: "localhost".to_string(),
            port: "3306".to_string(),
            db_name: "test_db".to_string(),
            sqlite_file_path: "".to_string(),
        };

        let sqlite_config = DbConfig {
            db_driver: "sqlite".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            db_host: "".to_string(),
            port: "".to_string(),
            db_name: "".to_string(),
            sqlite_file_path: "test_db".to_string(),
        };


        assert_eq!(DatabaseConnection::get_connection_url(&pg_config).unwrap(), "postgres://postgres:password@localhost:5432/test_db");
        assert_eq!(DatabaseConnection::get_connection_url(&mariadb_config).unwrap(), "mysql://root:password@localhost:3306/test_db");
        assert_eq!(DatabaseConnection::get_connection_url(&mysql_config).unwrap(), "mysql://root:password@localhost:3306/test_db");
        assert_eq!(DatabaseConnection::get_connection_url(&sqlite_config).unwrap(), "test_db");

        assert_ne!(DatabaseConnection::get_connection_url(&pg_config).unwrap(), "postgres://postgres:password@localhost:5432/test_db1");
        assert_ne!(DatabaseConnection::get_connection_url(&mariadb_config).unwrap(), "mysql://root:password@localhost:3306/test_db1");
        assert_ne!(DatabaseConnection::get_connection_url(&mysql_config).unwrap(), "mysql://root:password@localhost:3306/test_db1");
        assert_ne!(DatabaseConnection::get_connection_url(&sqlite_config).unwrap(), "test_db1");

        assert_eq!(DatabaseConnection::get_connection_url(&pg_config).unwrap(), "postgres://postgres:password@localhost:5432/test_db");
        assert_eq!(DatabaseConnection::get_connection_url(&mariadb_config).unwrap(), "mysql://root:password@localhost:3306/test_db");
        assert_eq!(DatabaseConnection::get_connection_url(&mysql_config).unwrap(), "mysql://root:password@localhost:3306/test_db");
        assert_eq!(DatabaseConnection::get_connection_url(&sqlite_config).unwrap(), "test_db");
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
        let absolute_path: PathBuf = std::env::current_exe().expect("Failed to get current executable path");
        let path: &str = absolute_path.parent().expect("Failed to get parent directory").to_str().expect("Failed to convert path to string");
        let file_path: String = format!("{}/test.db", path);

        if !std::path::Path::new(&file_path).exists() {
            std::fs::File::create(&file_path).expect("Failed to create SQLite file");
        }

        let config = DbConfig {
            db_driver: "sqlite".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            db_host: "".to_string(),
            port: "".to_string(),
            db_name: "".to_string(),
            sqlite_file_path: file_path.clone(),
        };

        let conn = DatabaseConnection::connect(&config).await;

        assert!(conn.is_success(), "Failed to connect to the database");
        assert_eq!(conn.is_ok(), true);
        assert_eq!(conn.is_err(), false);

        if let Err(e) = std::fs::remove_file(&file_path) {
            eprintln!("Failed to delete test database file: {}", e);
        }
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
        assert_eq!(get_drop_statement("sqlite", "table_name").unwrap(), "DROP TABLE IF EXISTS \"table_name\"");
        assert_eq!(get_drop_statement("mysql", "table_name").unwrap(), "DROP TABLE IF EXISTS `table_name`");
        assert_eq!(get_drop_statement("postgres", "table_name").unwrap(), "DROP TABLE IF EXISTS \"table_name\"");
        assert_eq!(get_drop_statement("unsupported", "table_name").unwrap_err(), "Unsupported database driver");

        assert_eq!(get_drop_statement("sqlite", "").unwrap(), "DROP TABLE IF EXISTS \"\"");
        assert_eq!(get_drop_statement("mysql", "").unwrap(), "DROP TABLE IF EXISTS ``");
        assert_eq!(get_drop_statement("postgres", "").unwrap(), "DROP TABLE IF EXISTS \"\"");
    }

    #[tokio::test]
    async fn test_get_insert_into_statement() {
        assert_eq!(get_insert_into_statement("sqlite", "table_name", "columns").unwrap(), "INSERT INTO \"table_name\" (columns) VALUES ");
        assert_eq!(get_insert_into_statement("mysql", "table_name", "columns").unwrap(), "INSERT INTO `table_name` (columns) VALUES ");
        assert_eq!(get_insert_into_statement("postgres", "table_name", "columns").unwrap(), "INSERT INTO \"table_name\" (columns) VALUES ");
        assert_eq!(get_insert_into_statement("unsupported", "table_name", "columns").unwrap_err(), "Unsupported database driver");

        assert_eq!(get_insert_into_statement("sqlite", "table_name", "").unwrap(), "INSERT INTO \"table_name\" () VALUES ");
        assert_eq!(get_insert_into_statement("mysql", "table_name", "").unwrap(), "INSERT INTO `table_name` () VALUES ");
        assert_eq!(get_insert_into_statement("postgres", "table_name", "").unwrap(), "INSERT INTO \"table_name\" () VALUES ");

        assert_eq!(get_insert_into_statement("sqlite", "table_name", "header1, header2").unwrap(), "INSERT INTO \"table_name\" (header1, header2) VALUES ");
        assert_eq!(get_insert_into_statement("mysql", "table_name", "header1, header2").unwrap(), "INSERT INTO `table_name` (header1, header2) VALUES ");
        assert_eq!(get_insert_into_statement("postgres", "table_name", "header1, header2").unwrap(), "INSERT INTO \"table_name\" (header1, header2) VALUES ");
    }

    #[tokio::test]
    async fn test_get_create_statement() {
        let snake_case_headers: Vec<String> = vec!["header1".to_string(), "header2".to_string()];
        assert_eq!(get_create_statement("sqlite", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)");
        assert_eq!(get_create_statement("mysql", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE `table_name` (header1 TEXT, header2 TEXT)");
        assert_eq!(get_create_statement("postgres", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)");
        assert_eq!(get_create_statement("unsupported", "table_name", &snake_case_headers).unwrap_err(), "Unsupported database driver");

        let snake_case_headers: Vec<String> = vec!["header1".to_string()];
        assert_eq!(get_create_statement("sqlite", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT)");
        assert_eq!(get_create_statement("mysql", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE `table_name` (header1 TEXT)");
        assert_eq!(get_create_statement("postgres", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT)");
    }

    #[tokio::test]
    async fn test_fast_insert() {
        let absolute_path: PathBuf = std::env::current_exe().expect("Failed to get current executable path");
        let path: &str = absolute_path.parent().expect("Failed to get parent directory").to_str().expect("Failed to convert path to string");
        let sqlite_file_path: String = format!("{}/test.db", path);

        if !std::path::Path::new(&sqlite_file_path).exists() {
            File::create(&sqlite_file_path).expect("Failed to create SQLite file");
        }

        let config = DbConfig {
            db_driver: "sqlite".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            db_host: "".to_string(),
            port: "".to_string(),
            db_name: "".to_string(),
            sqlite_file_path: sqlite_file_path.clone(),
        };

        let conn = DatabaseConnection::connect(&config).await;

        // Ensure the connection is valid
        assert!(conn.is_ok(), "Failed to connect to the database");

        let conn: DatabaseConnection = conn.unwrap();

        let csv_file_path: String = format!("{}/test.csv", path);
        let file: File = File::create(&csv_file_path).expect("Failed to create CSV file");
        let mut wtr: Writer<File> = Writer::from_writer(file);
        wtr.write_record(&["value1", "value2"]).expect("Failed to write record");
        wtr.write_record(&["value3", "value4"]).expect("Failed to write record");
        wtr.flush().expect("Failed to flush CSV writer");

        let snake_case_headers: Vec<String> = vec!["header1".to_string(), "header2".to_string()];
        let final_table_name: &str = "test_table";
        let db_driver: &str = "sqlite";

        let mut reader: Reader<File> = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(&csv_file_path).expect("Failed to open CSV file"));

        let result: Result<u64, String> = crate::fileflow::fast_insert::fast_insert(&conn, &mut reader, &snake_case_headers, final_table_name, db_driver).await;

        // Ensure the fast insert operation is successful
        assert!(result.is_ok(), "Failed to perform fast insert");

        // Ensure the number of rows inserted is as expected
        let inserted_count: u64 = result.unwrap();
        assert_eq!(inserted_count, 2, "Unexpected number of rows inserted");

        // Test table existence
        let query: &str = "SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'";
        let query_result: Result<SqliteRow, Error> = conn.fetch_one_sqlite(query).await;
        let query_result: SqliteRow = query_result.expect("Failed to query the database for table existence");
        let value: String = query_result.get("name");

        assert_eq!(value, "test_table");

        // Test fetching a row
        let query: &str = "SELECT header1,header2 FROM test_table LIMIT 1";
        let row: SqliteRow = conn.fetch_one_sqlite(query).await.expect("Failed to fetch row");
        let value1: String = row.get("header1");
        let value2: String = row.get("header2");

        assert_eq!(value1, "value1");
        assert_eq!(value2, "value2");


        if let Err(e) = std::fs::remove_file(&sqlite_file_path) {
            eprintln!("Failed to delete test database file: {}", e);
        }

        if let Err(e) = std::fs::remove_file(&csv_file_path) {
            eprintln!("Failed to delete test CSV file: {}", e);
        }
    }
}