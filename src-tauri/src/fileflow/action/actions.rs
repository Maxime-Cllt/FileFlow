// src/fileflow/action/actions.rs

use std::fs::{File, Metadata};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use csv::{Reader, ReaderBuilder};
use tauri::{command, State};
use tokio::sync::Mutex;
use crate::fileflow::fast_insert::fast_insert;
use crate::fileflow::database_connection::DatabaseConnection;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::save_config::SaveConfig;
use crate::fileflow::optimized_insert::optimized_insert;

pub struct DatabaseState(pub Mutex<Option<DatabaseConnection>>);

#[command]
pub async fn connect_to_database(state: State<'_, Arc<DatabaseState>>, config: DbConfig) -> Result<String, String> {
    let mut conn_guard = state.0.lock().await;

    if conn_guard.is_some() {
        return Err("Already connected to the database.".to_string());
    }

    match DatabaseConnection::connect(&config).await {
        Ok(connection) => {
            *conn_guard = Some(connection);
            Ok(format!(
                "Connected to {} database at {} with user {}",
                config.db_driver, config.db_host, config.username
            ))
        }
        Err(err) => Err(format!("Failed to connect to the database: {}", err)),
    }
}

#[command]
pub async fn insert_csv_data(state: State<'_, Arc<DatabaseState>>, csv: InsertConfig) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection.".to_string());
    }

    let conn: &DatabaseConnection = conn_guard.as_ref().unwrap();
    let start: Instant = Instant::now();

    let file: File = File::open(csv.file_path).unwrap();
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut headers: Vec<String> = reader.headers().unwrap().iter().map(|h| h.to_string()).collect();

    for i in 0..headers.len() {
        if headers[i].trim().len() == 0 {
            headers[i] = format!("column_{}", i + 1);
        }
    }

    let snake_case_headers: Vec<String> = headers.iter().map(|h| h.to_lowercase().replace(" ", "_")).collect();

    let line_count: u64;
    if csv.mode == "fast" {
        line_count = fast_insert(&conn, &mut reader, &snake_case_headers, &csv.table_name, &csv.db_driver).await?;
    } else {
        line_count = optimized_insert(&conn, &mut reader, &snake_case_headers, &csv.table_name, &csv.db_driver).await?;
    }

    Ok(format!("Data inserted successfully in {:.2?}, {} rows inserted in table {}", start.elapsed(), &line_count, &csv.table_name))
}

#[command]
pub async fn disconnect_from_database(state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
    let mut conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection to disconnect.".to_string());
    }

    let conn: DatabaseConnection = conn_guard.take().unwrap();
    conn.disconnect();

    Ok("Disconnected from the database.".to_string())
}

#[command]
pub async fn save_database_config(save: SaveConfig) -> Result<String, String> {
    let path: PathBuf = PathBuf::from("database_config.json");
    let _config = serde_json::to_string(&save).map_err(|e| format!("Failed to serialize config: {}", e))?;
    let file: File = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
    serde_json::to_writer_pretty(file, &save).map_err(|e| format!("Failed to write to file: {}", e))?;
    Ok(format!("Database configuration saved to file located at {}", path.display()))
}

#[command]
pub async fn load_database_config() -> Result<String, String> {
    let path: PathBuf = PathBuf::from("database_config.json");
    let file: File = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let config: SaveConfig = serde_json::from_reader(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let config_json: String = serde_json::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    Ok(config_json)
}

#[command]
pub async fn get_size_of_file(file_path: String) -> Result<String, String> {
    let metadata: Metadata = std::fs::metadata(&file_path).map_err(|e| format!("Failed to get metadata: {}", e))?;
    if !metadata.is_file() {
        return Err("Path is not a file".to_string());
    }
    if metadata.len() == 0 {
        return Err("File is empty".to_string());
    }
    let size: f64 = metadata.len() as f64 / 1024.0 / 1024.0;
    Ok(format!("{:.2} MB", size))
}
