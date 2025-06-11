use crate::fileflow::action::database_command::{fast_insert, optimized_insert};
use crate::fileflow::database::connection::Connection;
use crate::fileflow::enumeration::insertion_type::InsertionType;
use crate::fileflow::stuct::insert_config::InsertConfig;
use crate::fileflow::stuct::save_config::SaveConfig;
use crate::fileflow::stuct::string_formater::StringFormatter;
use crate::fileflow::utils::constants::DATABASE_CONFIG_FILE;
use crate::fileflow::utils::csv_utils::{find_separator, read_first_line};
use crate::fileflow::utils::fileflowlib::{get_all_saved_configs, save_config};
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
        return Err("Error: Connection is not established".into());
    }

    let connection: &Connection = conn_guard.as_ref().unwrap();
    let mut total_lines: u64 = 0; // Counter for the total number of lines inserted
    let start: Instant = Instant::now(); // Timer for the insertion process

    let file: File = File::open(&csv.file_path).expect("Failed to open file");
    let first_line: String = read_first_line(&csv.file_path).expect("Failed to read first line"); // Read the first line of the file to detect the separator
    let separator: char = find_separator(&first_line).expect("Failed to find separator"); // Separator detection of the file

    let final_columns_name: Vec<String> = StringFormatter::get_formated_column_names(
        &first_line
            .split(separator)
            .map(|s| StringFormatter::sanitize_column(s))
            .collect::<Vec<String>>(),
    );

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(u8::try_from(separator).unwrap())
        .has_headers(true)
        .from_reader(file);

    match csv.mode {
        InsertionType::Fast => {
            match fast_insert(
                connection,
                &mut reader,
                &final_columns_name,
                &csv.table_name,
                &csv.db_driver,
            )
            .await
            {
                Ok(lines) => {
                    total_lines += u64::from(lines);
                }
                Err(e) => {
                    return Err(format!("Error: Failed to insert data: {}", e.to_string()));
                }
            }
        }
        InsertionType::Optimized => {
            match optimized_insert(
                connection,
                &mut reader,
                &final_columns_name,
                &csv.table_name,
                &csv.db_driver,
            )
            .await
            {
                Ok(lines) => {
                    total_lines += u64::from(lines);
                }
                Err(e) => {
                    return Err(format!("Error: Failed to insert data: {}", e.to_string()));
                }
            }
        }
    };

    Ok(format!(
        "Inserted {total_lines} lines into {} tables in {:?} seconds",
        csv.table_name,
        start.elapsed(),
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

    save_config(&existing_configs, DATABASE_CONFIG_FILE)
        .map(|_| true)
        .map_err(|_| false)
}

#[command]
pub async fn get_all_database_configs_name() -> Result<String, bool> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE); // Get all saved configs
    let configs_names: Vec<String> = configs.iter().map(|c| c.config_name.clone()).collect(); // Get only the names
    let configs_json: String = match serde_json::to_string(&configs_names) {
        Ok(json) => json, // Convert to json string
        Err(_) => return Err(false),
    };
    Ok(configs_json)
}

#[command]
pub async fn load_database_config_by_name(name: String) -> Result<String, bool> {
    let configs: Vec<SaveConfig> = get_all_saved_configs(DATABASE_CONFIG_FILE); // Get all saved configs
    for config in configs.iter() {
        // Find the config with the given name
        if config.config_name == name {
            return match serde_json::to_string(&config) {
                Ok(json) => Ok(json), // Convert to json string
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
        return Err(false);
    }

    save_config(&new_configs, DATABASE_CONFIG_FILE)
        .map(|_| true)
        .map_err(|_| false)
}

#[command]
pub async fn get_size_of_file(file_path: String) -> Result<String, bool> {
    let metadata: Metadata = std::fs::metadata(&file_path).map_err(|_| false)?;
    if !metadata.is_file() {
        return Err(false);
    }
    if metadata.len() == 0 {
        return Err(false);
    }
    let size: f64 = metadata.len() as f64 / 1024.0 / 1024.0;
    Ok(format!("{size:.2} MB"))
}
