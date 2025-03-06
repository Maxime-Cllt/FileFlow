use crate::fileflow::database::connection::Connection;
use crate::fileflow::action::fast_insert::fast_insert;
use crate::fileflow::action::optimized_insert::optimized_insert;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::save_config::SaveConfig;
use crate::fileflow::utils::constants::DATABASE_CONFIG_FILE;
use crate::fileflow::utils::fileflowlib::{
    find_separator, get_all_saved_configs, get_formated_column_names, read_first_line, save_config,
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
        return Err("No active database connection.".into());
    }

    let connection: &Connection = conn_guard.as_ref().unwrap();
    let start: Instant = Instant::now();

    let file: File = File::open(&csv.file_path).unwrap();

    let first_line: String = read_first_line(&csv.file_path).unwrap();
    let separator: char = find_separator(&first_line)?;
    let final_columns_name: Vec<String> = get_formated_column_names(
        first_line
            .split(separator)
            .map(|s| s.replace("\"", ""))
            .collect(),
    );

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(separator as u8)
        .has_headers(true)
        .from_reader(file);

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
        line_count,
        csv.table_name
    ))
}

#[command]
pub async fn save_database_config(save: SaveConfig) -> Result<bool, bool> {
    let mut existing_configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);

    for config in existing_configs.iter() {
        if config.config_name == save.config_name {
            return Err(false);
        }
    }
    existing_configs.push(save);

    match save_config(&existing_configs, DATABASE_CONFIG_FILE) {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}

#[command]
pub async fn get_all_database_configs_name() -> Result<String, bool> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);
    let configs_names: Vec<String> = configs.iter().map(|c| c.config_name.clone()).collect();
    let configs_json: String = match serde_json::to_string(&configs_names) {
        Ok(json) => json,
        Err(_) => return Err(false),
    };
    Ok(configs_json)
}

#[command]
pub async fn load_database_config_by_name(name: String) -> Result<String, bool> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);
    for config in configs.iter() {
        if config.config_name == name {
            return match serde_json::to_string(&config) {
                Ok(json) => Ok(json),
                Err(_) => Err(false),
            };
        }
    }
    Err(false)
}

#[command]
pub async fn delete_database_config(name: String) -> Result<bool, bool> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE);

    let mut new_configs: Vec<SaveConfig> = Vec::new();
    let mut found: bool = false;

    for config in configs.iter() {
        if config.config_name != name {
            new_configs.push(config.clone());
        } else {
            found = true;
        }
    }

    if !found {
        Err(false)?;
    }

    match save_config(&new_configs, DATABASE_CONFIG_FILE) {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}

#[command]
pub async fn get_size_of_file(file_path: String) -> Result<String, String> {
    let metadata: Metadata =
        std::fs::metadata(&file_path).map_err(|e| format!("Failed to get metadata: {e}"))?;
    if !metadata.is_file() {
        return Err("Path is not a file".into());
    }
    if metadata.len() == 0 {
        return Err("File is empty".into());
    }
    let size: f64 = metadata.len() as f64 / 1024.0 / 1024.0;
    Ok(format!("{size:.2} MB"))
}
