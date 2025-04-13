use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::save_config::SaveConfig;
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

/// Get a test PostgreSQL configuration
pub fn get_test_pg_config() -> DbConfig {
    DbConfig {
        db_driver: DatabaseEngine::Postgres,
        username: "postgres".into(),
        password: String::from("password"),
        db_host: "localhost".into(),
        port: "5432".into(),
        db_name: "test_db".into(),
        sqlite_file_path: String::new(),
    }
}

/// Get a test SQLite configuration
pub fn get_test_sqlite_config(str: String) -> DbConfig {
    DbConfig {
        db_driver: DatabaseEngine::SQLite,
        username: String::new(),
        password: String::new(),
        db_host: String::new(),
        port: String::new(),
        db_name: String::new(),
        sqlite_file_path: if str.is_empty() {
            "test_db.db".into()
        } else {
            str
        },
    }
}

/// Get a test MySQL configuration
pub fn get_test_mysql_config() -> DbConfig {
    DbConfig {
        db_driver: DatabaseEngine::MySQL,
        username: "root".into(),
        password: "password".into(),
        db_host: "localhost".into(),
        port: "3306".into(),
        db_name: "test_db".into(),
        sqlite_file_path: String::new(),
    }
}

/// Get a test MariaDB configuration
pub fn get_test_maridb_config() -> DbConfig {
    DbConfig {
        db_driver: DatabaseEngine::MariaDB,
        username: String::from("root"),
        password: String::from("password"),
        db_host: "localhost".into(),
        port: "3306".into(),
        db_name: "test_db".into(),
        sqlite_file_path: String::new(),
    }
}
/// Get a test save configuration
pub fn get_test_save_config(config_name: &str) -> SaveConfig {
    SaveConfig {
        config_name: config_name.into(),
        db_driver: DatabaseEngine::SQLite,
        db_host: String::new(),
        port: String::new(),
        username: String::new(),
        password: String::new(),
        db_name: "test_db".into(),
        sqlite_file_path: String::new(),
    }
}

/// Delete the configuration file with the given file name
pub fn delete_config_file(config_file_name: &str) -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new(&config_file_name).exists() {
        println!("File does not exist");
        Err("Failed to create SQLite file")?;
    }
    std::fs::remove_file(config_file_name).expect("Failed to remove SQLite file");
    Ok(())
}

/// Create a test SQLite database file if it does not exist and return the file path
pub fn create_test_db(db_name: &str) -> String {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");
    let file_path: String = format!("{path}/{db_name}.db");

    if !std::path::Path::new(&file_path).exists() {
        File::create(&file_path).expect("Failed to create SQLite file");
    }

    file_path
}

/// Remove the test SQLite database file if it exists from its path
pub fn remove_test_db(db_name: &str) -> Result<(), Box<dyn Error>> {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");
    let file_path: String = format!("{path}/{db_name}.db");

    if !std::path::Path::new(&file_path).exists() {
        println!("File does not exist");
        return Ok(());
    }

    std::fs::remove_file(file_path).expect("Failed to remove SQLite file");
    Ok(())
}

/// Generate a CSV file with the given file name and return the file path
pub fn generate_csv_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");

    let csv_file_path: String = format!("{path}/{file_name}.csv");
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
pub fn remove_csv_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let absolute_path: PathBuf =
        std::env::current_exe().expect("Failed to get current executable path");
    let path: &str = absolute_path
        .parent()
        .expect("Failed to get parent directory")
        .to_str()
        .expect("Failed to convert path to string");
    let csv_file_path: String = format!("{path}/{file_name}.csv");

    if std::path::Path::new(&csv_file_path).exists() {
        std::fs::remove_file(&csv_file_path).expect("Failed to remove CSV file");
        Ok(())
    } else {
        println!("File does not exist");
        Err("Failed to remove CSV file")?
    }
}
