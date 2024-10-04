// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod optimized_insert;
mod fast_insert;

use crate::fast_insert::fast_insert;
use crate::optimized_insert::optimized_insert;
use csv::ReaderBuilder;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::command;
use tauri::State;
use tokio::sync::Mutex;
mod database_connection;
use crate::database_connection::db_config::DbConfig;
use crate::database_connection::insert_config::InsertConfig;
use crate::database_connection::save_config::SaveConfig;
use crate::database_connection::DatabaseConnection;

struct DatabaseState(Mutex<Option<DatabaseConnection>>);

#[command]
async fn connect_to_database(config: DbConfig, state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
    let mut conn_guard = state.0.lock().await;

    // Check if there is already an active connection
    if conn_guard.is_some() {
        return Err("Already connected to the database.".to_string());
    }

    match DatabaseConnection::connect(&config).await {
        Ok(connection) => {
            *conn_guard = Some(connection); // Store the connection in the shared state
            Ok(format!(
                "Connected to {} database at {} with user {}",
                config.db_driver, config.db_host, config.username
            ))
        }
        Err(err) => Err(format!("Failed to connect to the database: {}", err)),
    }
}

#[tauri::command]
async fn insert_csv_data(csv: InsertConfig, state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection.".to_string());
    }

    let conn = conn_guard.as_ref().unwrap();
    let start = std::time::Instant::now();

    let file = File::open(csv.file_path).unwrap();
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers: Vec<String> = reader.headers().unwrap().iter().map(|h| h.to_string()).collect();
    let snake_case_headers: Vec<String> = headers.iter().map(|h| h.to_lowercase().replace(" ", "_")).collect();


    let mut line_count = 0;
    if csv.mode == "fast" {
        line_count = fast_insert(&conn, &mut reader, &snake_case_headers, &csv.table_name).await?;
    } else {
        optimized_insert(&conn, &mut reader, &snake_case_headers, &csv.table_name).await?;
    }

    let duration = start.elapsed();
    let ok = format!("Data inserted successfully in {:.2?}, {} rows inserted.", duration, line_count);
    Ok(ok)
}

#[command]
async fn disconnect_from_database(state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
    let mut conn_guard = state.0.lock().await;

    // Check if there's an active connection to disconnect
    if conn_guard.is_none() {
        return Err("No active database connection to disconnect.".to_string());
    }

    // Drop the connection
    let conn = conn_guard.take().unwrap();
    conn.disconnect();

    // Take ownership of the connection and replace it with None
    conn_guard.take();
    Ok("Disconnected from the database.".to_string())
}

#[tauri::command]
async fn save_database_config(save: SaveConfig) -> Result<String, String> {
    let path = PathBuf::from("database_config.json");
    let _config = serde_json::to_string(&save).map_err(|e| format!("Failed to serialize config: {}", e))?;
    let file = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
    serde_json::to_writer_pretty(file, &save).map_err(|e| format!("Failed to write to file: {}", e))?;
    Ok(format!("Database configuration saved to file located at {}", path.display()))
}

#[tauri::command]
async fn load_database_config() -> Result<String, String> {
    let path = PathBuf::from("database_config.json");
    let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let config: SaveConfig = serde_json::from_reader(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let config_json = serde_json::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    Ok(config_json)
}

#[tauri::command]
async fn get_size_of_file(file_path: String) -> Result<String, String> {
    let metadata = std::fs::metadata(file_path).map_err(|e| format!("Failed to get metadata: {}", e))?;
    let size = metadata.len() as f64 / 1024.0 / 1024.0;
    Ok(format!("{:.2} MB", size))
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(DatabaseState(Mutex::new(None)))) // Initialize the shared state with Arc and tokio Mutex
        .invoke_handler(tauri::generate_handler![
            connect_to_database,
            disconnect_from_database,
            insert_csv_data,
            save_database_config,
            load_database_config,
            get_size_of_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
