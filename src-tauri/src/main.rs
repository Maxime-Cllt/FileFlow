// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod optimized_insert;
mod fast_insert;
mod fileflow;

use crate::fast_insert::fast_insert;
use crate::optimized_insert::optimized_insert;
use csv::{Reader, ReaderBuilder};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Instant};
use tauri::command;
use tauri::State;
use tokio::sync::Mutex;
use fileflow::db_config::DbConfig;
use fileflow::insert_config::InsertConfig;
use fileflow::save_config::SaveConfig;
use fileflow::database_connection::DatabaseConnection;

struct DatabaseState(Mutex<Option<DatabaseConnection>>);

#[command]
async fn connect_to_database(config: DbConfig, state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
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
async fn insert_csv_data(csv: InsertConfig, state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection.".to_string());
    }

    let conn: &DatabaseConnection = conn_guard.as_ref().unwrap();
    let start: Instant = Instant::now();

    let file: File = File::open(csv.file_path).unwrap();
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers: Vec<String> = reader.headers().unwrap().iter().map(|h| h.to_string()).collect();
    let snake_case_headers: Vec<String> = headers.iter().map(|h| h.to_lowercase().replace(" ", "_")).collect();

    let line_count: u64;
    if csv.mode == "fast" {
        line_count = fast_insert(&conn, &mut reader, &snake_case_headers, &csv.table_name, &csv.db_driver).await?;
    } else {
        line_count = optimized_insert(&conn, &mut reader, &snake_case_headers, &csv.table_name, &csv.db_driver).await?;
    }

    Ok(format!("Data inserted successfully in {:.2?}, {} rows inserted in table {}", start.elapsed(), &line_count, csv.table_name))
}

#[command]
async fn disconnect_from_database(state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
    let mut conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection to disconnect.".to_string());
    }

    let conn = conn_guard.take().unwrap();
    conn.disconnect();

    conn_guard.take();
    Ok("Disconnected from the database.".to_string())
}

#[command]
async fn save_database_config(save: SaveConfig) -> Result<String, String> {
    let path: PathBuf = PathBuf::from("database_config.json");

    let _ = match serde_json::to_string(&save) {
        Ok(config) => config,
        Err(e) => {
            println!("Failed to serialize config: {}", e);
            return Err(format!("Failed to serialize config: {}", e));
        }
    };

    let file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to create file: {}", e);
            return Err(format!("Failed to create file: {}", e));
        }
    };

    if let Err(e) = serde_json::to_writer_pretty(file, &save) {
        println!("Failed to write to file: {}", e);
        return Err(format!("Failed to write to file: {}", e));
    }

    Ok(format!(
        "Database configuration saved to file located at {}",
        path.display()
    ))
}

#[command]
async fn load_database_config() -> Result<String, String> {
    let path = PathBuf::from("database_config.json");
    let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let config: SaveConfig = serde_json::from_reader(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let config_json = serde_json::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    Ok(config_json)
}

#[tauri::command]
async fn get_size_of_file(file_path: String) -> Result<String, String> {
    let metadata = std::fs::metadata(file_path).unwrap();
    match metadata.is_file() {
        false => return Err("Path is not a file".to_string()),
        true => {
            if metadata.len() == 0 {
                return Err("File is empty".to_string());
            }
        }
    }
    let size: f64 = metadata.len() as f64 / 1024.0 / 1024.0;
    Ok(format!("{:.2} MB", size))
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(DatabaseState(Mutex::new(None))))
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
