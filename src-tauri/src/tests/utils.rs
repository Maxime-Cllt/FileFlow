use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::utils::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

/// Get a test PostgreSQL configuration
pub fn get_test_pg_config() -> DbConfig {
    DbConfig {
        db_driver: POSTGRES.to_string(),
        username: POSTGRES.to_string(),
        password: "password".to_string(),
        db_host: "localhost".to_string(),
        port: "5432".to_string(),
        db_name: "test_db".to_string(),
        sqlite_file_path: "".to_string(),
    }
}

/// Get a test SQLite configuration
pub fn get_test_sqlite_config(str: String) -> DbConfig {
    let mut str: String = str;
    if str.is_empty() {
        str = "test_db.db".to_string();
    }
    DbConfig {
        db_driver: SQLITE.to_string(),
        username: "".to_string(),
        password: "".to_string(),
        db_host: "".to_string(),
        port: "".to_string(),
        db_name: "".to_string(),
        sqlite_file_path: str,
    }
}

/// Get a test MySQL configuration
pub fn get_test_mysql_config() -> DbConfig {
    DbConfig {
        db_driver: MYSQL.to_string(),
        username: "root".to_string(),
        password: "password".to_string(),
        db_host: "localhost".to_string(),
        port: "3306".to_string(),
        db_name: "test_db".to_string(),
        sqlite_file_path: "".to_string(),
    }
}

/// Get a test MariaDB configuration
pub fn get_test_maridb_config() -> DbConfig {
    DbConfig {
        db_driver: MARIADB.to_string(),
        username: "root".to_string(),
        password: "password".to_string(),
        db_host: "localhost".to_string(),
        port: "3306".to_string(),
        db_name: "test_db".to_string(),
        sqlite_file_path: "".to_string(),
    }
}

/// Create a test SQLite database file if it does not exist and return the file path
pub fn create_test_db(db_name: String) -> String {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");
    let file_path: String = format!("{}/{}.db", path, db_name);

    if !std::path::Path::new(&file_path).exists() {
        File::create(&file_path).expect("Failed to create SQLite file");
    }

    file_path
}

/// Remove the test SQLite database file if it exists from its path
pub fn remove_test_db(db_name: String) -> Result<(), Box<dyn Error>> {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");
    let file_path: String = format!("{}/{}.db", path, db_name);

    if !std::path::Path::new(&file_path).exists() {
        println!("File does not exist");
        Err("Failed to create SQLite file")?;
    }
    std::fs::remove_file(&file_path).expect("Failed to remove SQLite file");
    Ok(())
}

/// Generate a CSV file with the given file name and return the file path
pub fn generate_csv_file(file_name: String) -> Result<String, Box<dyn Error>> {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");

    let csv_file_path: String = format!("{}/{}.csv", path, file_name);
    let file: File = File::create(&csv_file_path).expect("Failed to create CSV file");
    let mut wtr: Writer<File> = Writer::from_writer(file);
    wtr.write_record(["header1", "header2"])
        .expect("Failed to write record");
    wtr.write_record(["value1", "value2"])
        .expect("Failed to write record");
    wtr.write_record(["value3", "value4"])
        .expect("Failed to write record");
    wtr.flush().expect("Failed to flush CSV writer");

    // check if file exists
    if !std::path::Path::new(&csv_file_path).exists() {
        println!("File does not exist");
        Err("Failed to create CSV file")?;
    }

    Ok(csv_file_path)
}

/// Remove the CSV file with the given file name if it exists
pub fn remove_csv_file(file_name: String) -> Result<(), Box<dyn Error>> {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");
    let csv_file_path: String = format!("{}/{}.csv", path, file_name);

    if std::path::Path::new(&csv_file_path).exists() {
        std::fs::remove_file(&csv_file_path).expect("Failed to remove CSV file");
        Ok(())
    } else {
        println!("File does not exist");
        Err("Failed to remove CSV file")?
    }
}
