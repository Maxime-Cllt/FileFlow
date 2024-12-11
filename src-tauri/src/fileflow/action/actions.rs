// src/fileflow/action/actions.rs

use crate::fileflow::database::connection::DatabaseConnection;
use crate::fileflow::fast_insert::fast_insert;
use crate::fileflow::fileflowlib::{
    detect_separator_in_file, get_create_statement_with_fixed_size, get_drop_statement,
    get_formated_column_names,
};
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
        Err(err) => Err(format!("Failed to connect to the database: {err}")),
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

    let connection: &DatabaseConnection = conn_guard.as_ref().unwrap();
    let start: Instant = Instant::now();

    let file: File = File::open(&csv.file_path).unwrap();
    let separator: char = detect_separator_in_file(&csv.file_path).unwrap();

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(separator as u8)
        .has_headers(true)
        .from_reader(file);

    let final_columns_name: Vec<String> = get_formated_column_names(
        reader
            .headers()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect(),
    );

    let line_count: u64 = if csv.mode == "fast" {
        fast_insert(
            connection,
            &mut reader,
            &final_columns_name,
            &csv.table_name,
            &csv.db_driver,
        )
        .await?
    } else {
        optimized_insert(
            connection,
            &mut reader,
            &final_columns_name,
            &csv.table_name,
            &csv.db_driver,
        )
        .await?
    };

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
        serde_json::to_string(&save).map_err(|e| format!("Failed to serialize config: {e}"))?;
    let file: File = File::create(&path).map_err(|e| format!("Failed to create file: {e}"))?;
    serde_json::to_writer_pretty(file, &save)
        .map_err(|e| format!("Failed to write to file: {e}"))?;
    Ok(format!(
        "Database configuration saved to file located at {}",
        path.display()
    ))
}

#[command]
pub async fn load_database_config() -> Result<String, String> {
    let path: PathBuf = PathBuf::from("database_config.json");
    let file: File = File::open(&path).map_err(|e| format!("Failed to open file: {e}"))?;
    let config: SaveConfig =
        serde_json::from_reader(file).map_err(|e| format!("Failed to read file: {e}"))?;
    let config_json: String =
        serde_json::to_string(&config).map_err(|e| format!("Failed to serialize config: {e}"))?;
    Ok(config_json)
}

#[command]
pub async fn get_size_of_file(file_path: String) -> Result<String, String> {
    let metadata: Metadata =
        std::fs::metadata(&file_path).map_err(|e| format!("Failed to get metadata: {e}"))?;
    if !metadata.is_file() {
        return Err("Path is not a file".to_string());
    }
    if metadata.len() == 0 {
        return Err("File is empty".to_string());
    }
    let size: f64 = metadata.len() as f64 / 1024.0 / 1024.0;
    Ok(format!("{size:.2} MB"))
}

#[command]
pub async fn generate_load_data_sql(load: GenerateLoadData) -> Result<String, String> {
    if load.file_path.is_empty() {
        return Err("File path is empty".to_string());
    }

    let file: File =
        File::open(&load.file_path).map_err(|e| format!("Failed to open file: {e}"))?;
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_reader(file);

    let final_columns_name: Vec<String> = get_formated_column_names(
        reader
            .headers()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect(),
    );

    let mut columns_size_map: HashMap<&str, usize> = HashMap::new();
    for header in &final_columns_name {
        columns_size_map.insert(header, 0);
    }

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        for (i, value) in record.iter().enumerate() {
            let value: String = value.trim().to_string();
            let max_length: &mut usize = columns_size_map
                .get_mut(final_columns_name[i].as_str())
                .unwrap();
            if value.len() > *max_length {
                *max_length = value.len() + 1;
            }
        }
    }

    let separator: char = detect_separator_in_file(&load.file_path).unwrap();

    // Generate SQL
    let mut sql: String = String::new();

    // Delete table if exists
    sql.push_str(get_drop_statement(&load.db_driver, &load.table_name)?.as_str());
    sql.push(';');
    sql.push_str("\n\n");

    // Create table with fixed size
    sql.push_str(
        get_create_statement_with_fixed_size(
            &load.db_driver,
            &load.table_name,
            &columns_size_map,
            &final_columns_name,
        )?
        .as_str(),
    );
    sql.push_str("\n\n");

    // Load data into table from file
    sql.push_str("LOAD DATA INFILE '");
    sql.push_str(load.file_path.as_str());
    sql.push_str("'\nINTO TABLE ");
    sql.push_str(load.table_name.as_str());
    sql.push_str("\nCHARACTER SET utf8\n");
    sql.push_str("FIELDS TERMINATED BY '");
    sql.push(separator);
    sql.push_str("'\n");
    sql.push_str("ENCLOSED BY '\"'\nLINES TERMINATED BY '\\n'\nIGNORE 1 ROWS (");
    sql.push_str(&final_columns_name.join(", "));
    sql.push_str(");");

    Ok(sql)
}

#[command]
pub async fn execute_sql(
    state: State<'_, Arc<DatabaseState>>,
    sql: String,
) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection.".to_string());
    }

    let connection: &DatabaseConnection = conn_guard.as_ref().unwrap();
    let start: Instant = Instant::now();

    for query in sql.split(';') {
        if query.trim().is_empty() {
            continue;
        }
        connection
            .query(query)
            .await
            .map_err(|e| format!("Failed to execute query: {e}"))?;
    }

    Ok(format!(
        "Query executed successfully in {:.2?}",
        start.elapsed()
    ))
}

#[command]
pub async fn is_connected(state: State<'_, Arc<DatabaseState>>) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Ok("false".to_string());
    }

    let connection: &DatabaseConnection = conn_guard.as_ref().unwrap();
    let db_config: &DbConfig = connection.get_db_config();
    Ok(serde_json::to_string(&db_config).unwrap())
}
