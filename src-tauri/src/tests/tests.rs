
// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fileflow::stuct::db_config::DbConfig;
    use crate::fileflow::stuct::insert_config::InsertConfig;
    use crate::fileflow::stuct::save_config::SaveConfig;
    use crate::fileflow::fileflow::{get_drop_statement, get_insert_into_statement, get_create_statement};
    use crate::fileflow::database_connection::DatabaseConnection;

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
        let snake_case_headers = vec!["header1".to_string(), "header2".to_string()];
        assert_eq!(get_create_statement("sqlite", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)");
        assert_eq!(get_create_statement("mysql", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE `table_name` (header1 TEXT, header2 TEXT)");
        assert_eq!(get_create_statement("postgres", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)");
        assert_eq!(get_create_statement("unsupported", "table_name", &snake_case_headers).unwrap_err(), "Unsupported database driver");

        let snake_case_headers = vec!["header1".to_string()];
        assert_eq!(get_create_statement("sqlite", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT)");
        assert_eq!(get_create_statement("mysql", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE `table_name` (header1 TEXT)");
        assert_eq!(get_create_statement("postgres", "table_name", &snake_case_headers).unwrap(), "CREATE TABLE \"table_name\" (header1 TEXT)");
    }
}