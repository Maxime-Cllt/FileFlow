// src/fileflow/action/actions.rs

use crate::fileflow::database_connection::DatabaseConnection;
use crate::fileflow::fast_insert::fast_insert;
use crate::fileflow::fileflow::{get_create_statement_with_fixed_size, get_formated_column_names};
use crate::fileflow::optimized_insert::optimized_insert;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::load_data_struct::GenerateLoadData;
use crate::fileflow::stuct::save_config::SaveConfig;
use csv::{Reader, ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs::{File, Metadata};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tauri::{command, State};
use tokio::sync::Mutex;

pub struct DatabaseState(pub Mutex<Option<DatabaseConnection>>);

#[command]
pub async fn connect_to_database(
    state: State<'_, Arc<DatabaseState>>,
    config: DbConfig,
) -> Result<String, String> {
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
pub async fn insert_csv_data(
    state: State<'_, Arc<DatabaseState>>,
    csv: InsertConfig,
) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection.".to_string());
    }

    let conn: &DatabaseConnection = conn_guard.as_ref().unwrap();
    let start: Instant = Instant::now();

    let file: File = File::open(csv.file_path).unwrap();
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_reader(file);

    let snake_case_headers: Vec<String> = get_formated_column_names(
        reader
            .headers()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect(),
    );

    let line_count: u64;
    if csv.mode == "fast" {
        line_count = fast_insert(
            &conn,
            &mut reader,
            &snake_case_headers,
            &csv.table_name,
            &csv.db_driver,
        )
        .await?;
    } else {
        line_count = optimized_insert(
            &conn,
            &mut reader,
            &snake_case_headers,
            &csv.table_name,
            &csv.db_driver,
        )
        .await?;
    }

    Ok(format!(
        "Data inserted successfully in {:.2?}, {} rows inserted in table {}",
        start.elapsed(),
        &line_count,
        &csv.table_name
    ))
}

#[command]
pub async fn disconnect_from_database(
    state: State<'_, Arc<DatabaseState>>,
) -> Result<String, String> {
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
    let _config =
        serde_json::to_string(&save).map_err(|e| format!("Failed to serialize config: {}", e))?;
    let file: File = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
    serde_json::to_writer_pretty(file, &save)
        .map_err(|e| format!("Failed to write to file: {}", e))?;
    Ok(format!(
        "Database configuration saved to file located at {}",
        path.display()
    ))
}

#[command]
pub async fn load_database_config() -> Result<String, String> {
    let path: PathBuf = PathBuf::from("database_config.json");
    let file: File = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let config: SaveConfig =
        serde_json::from_reader(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let config_json: String =
        serde_json::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    Ok(config_json)
}

#[command]
pub async fn get_size_of_file(file_path: String) -> Result<String, String> {
    let metadata: Metadata =
        std::fs::metadata(&file_path).map_err(|e| format!("Failed to get metadata: {}", e))?;
    if !metadata.is_file() {
        return Err("Path is not a file".to_string());
    }
    if metadata.len() == 0 {
        return Err("File is empty".to_string());
    }
    let size: f64 = metadata.len() as f64 / 1024.0 / 1024.0;
    Ok(format!("{:.2} MB", size))
}

#[command]
pub async fn generate_load_data_sql(load_data_config: &GenerateLoadData) -> Result<String, String> {
    if load_data_config.file_path.is_empty() {
        return Err("File path is empty".to_string());
    }

    let file: File = File::open(&load_data_config.file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_reader(file);

    let formatted_headers: Vec<String> = get_formated_column_names(
        reader
            .headers()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect(),
    );

    let mut size_map: HashMap<&str, usize> = HashMap::new();
    for header in &formatted_headers {
        size_map.insert(header, 0);
    }

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        for i in 0..record.len() {
            let value: String = record.get(i).unwrap().trim().to_string();
            let max_length: &mut usize = size_map.get_mut(formatted_headers[i].as_str()).unwrap();
            if value.len() > *max_length {
                *max_length = value.len() + 1;
            }
        }
    }

    let mut sql: String = String::new();
    sql.push_str(
        get_create_statement_with_fixed_size(
            &load_data_config.db_driver,
            &load_data_config.table_name,
            &size_map,
            &formatted_headers,
        )
        .unwrap()
        .as_str(),
    );
    sql.push_str(";\n");

    sql.push_str("LOAD DATA LOCAL INFILE '");
    sql.push_str(load_data_config.file_path.as_str());
    sql.push_str("' INTO TABLE ");
    sql.push_str(load_data_config.table_name.as_str());
    sql.push_str(
        " FIELDS TERMINATED BY ',' ENCLOSED BY '\"' LINES TERMINATED BY '\\n' IGNORE 1 ROWS (",
    );
    sql.push_str(&formatted_headers.join(", "));
    sql.push_str(");");

    Ok(sql)
}
