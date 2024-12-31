// src/fileflow/action/actions.rs

use crate::fileflow::database::connection::Connection;
use crate::fileflow::fast_insert::fast_insert;
use crate::fileflow::optimized_insert::optimized_insert;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::save_config::SaveConfig;
use crate::fileflow::utils::constants::DATABASE_CONFIG_FILE;
use crate::fileflow::utils::fileflowlib::{
    detect_separator_in_file, get_all_saved_configs, get_formated_column_names, save_config,
};
use csv::{Reader, ReaderBuilder};
use std::fs::{File, Metadata};
use std::sync::Arc;
use std::time::Instant;
use tauri::{command, State};
use tokio::sync::Mutex;

pub struct DatabaseState(pub Mutex<Option<Connection>>);

#[command]
pub async fn insert_csv_data(
    state: State<'_, Arc<DatabaseState>>,
    csv: InsertConfig,
) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection.".to_string());
    }

    let connection: &Connection = conn_guard.as_ref().unwrap();
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
pub async fn save_database_config(save: SaveConfig) -> Result<String, String> {
    let mut existing_configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);

    for config in &existing_configs {
        if config.config_name == save.config_name {
            return Err("Database configuration already exists".to_string());
        }
    }
    let config_names: &str = &save.config_name.clone();
    existing_configs.push(save);
    save_config(&existing_configs, DATABASE_CONFIG_FILE)
        .map_err(|e| format!("Failed to save new configs: {e}"))?;

    Ok(format!(
        "Database configuration {config_names} added successfully",
    ))
}

#[command]
pub async fn get_all_database_configs_name() -> Result<String, String> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);
    let configs_names: Vec<String> = configs.iter().map(|c| c.config_name.clone()).collect();
    let configs_json: String = serde_json::to_string(&configs_names)
        .map_err(|e| format!("Failed to serialize configs: {e}"))?;
    Ok(configs_json)
}

#[command]
pub async fn load_database_config_by_name(name: String) -> Result<String, String> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);
    for config in &configs {
        if config.config_name == name {
            return serde_json::to_string(&config)
                .map_err(|e| format!("Failed to serialize config: {e}"));
        }
    }
    Err("Database configuration not found".to_string())
}

#[command]
pub async fn delete_database_config(name: String) -> Result<String, String> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);

    let mut new_configs: Vec<SaveConfig> = Vec::new();
    let mut found: bool = false;

    for config in &configs {
        if config.config_name != name {
            new_configs.push(config.clone());
        } else {
            found = true;
        }
    }

    if !found {
        Err("Database configuration not found".to_string())?;
    }

    save_config(&new_configs, DATABASE_CONFIG_FILE)
        .map_err(|e| format!("Failed to save new configs: {e}"))?;

    Ok(format!("Database configuration {name} deleted"))
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
