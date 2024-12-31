use crate::fileflow::action::actions::DatabaseState;
use crate::fileflow::database::connection::Connection;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::load_data_struct::GenerateLoadData;
use crate::fileflow::utils::constants::{MARIADB, MYSQL, POSTGRES};
use crate::fileflow::utils::fileflowlib::{detect_separator_in_file, get_formated_column_names};
use crate::fileflow::utils::sql::{get_create_statement_with_fixed_size, get_drop_statement};
use csv::{Reader, ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use std::time::Instant;
use tauri::{command, State};

#[command]
pub async fn connect_to_database(
    state: State<'_, Arc<DatabaseState>>,
    config: DbConfig,
) -> Result<String, String> {
    let mut conn_guard = state.0.lock().await;

    if conn_guard.is_some() {
        return Err("Already connected to the database.".to_string());
    }

    match Connection::connect(&config).await {
        Ok(connection) => {
            *conn_guard = Some(connection);

            let message: String = match config.db_driver.as_str() {
                MYSQL | POSTGRES | MARIADB => format!(
                    "Connected to {} database at {} with user {}",
                    config.db_driver, config.db_host, config.username
                ),
                _ => format!("Connected to {} database", config.db_driver),
            };
            Ok(message)
        }
        Err(err) => Err(format!("Failed to connect to the database: {err}")),
    }
}

#[command]
pub async fn disconnect_from_database(
    state: State<'_, Arc<DatabaseState>>,
) -> Result<String, String> {
    let mut conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection to disconnect.".to_string());
    }

    let conn: Connection = conn_guard.take().unwrap();
    conn.disconnect();

    Ok("Disconnected from the database.".to_string())
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

    let connection: &Connection = conn_guard.as_ref().unwrap();
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

    let connection: &Connection = conn_guard.as_ref().unwrap();
    let db_config: &DbConfig = connection.get_db_config();

    Ok(serde_json::to_string(db_config).unwrap())
}
