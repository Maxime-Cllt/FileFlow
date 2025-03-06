use crate::fileflow::action::actions::DatabaseState;
use crate::fileflow::database::connection::Connection;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::load_data_struct::GenerateLoadData;
use crate::fileflow::utils::constants::SQLITE;
use crate::fileflow::utils::fileflowlib::{
    build_load_data, find_separator, get_formated_column_names, read_first_line,
};
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
) -> Result<bool, String> {
    let mut conn_guard = state.0.lock().await;

    if conn_guard.is_some() {
        return Err("Already connected to the database.".into());
    }

    match Connection::connect(&config).await {
        Ok(connection) => {
            *conn_guard = Some(connection);
            Ok(true)
        }
        Err(err) => Err(format!("Failed to connect to the database: {err}")),
    }
}

#[command]
pub async fn disconnect_from_database(
    state: State<'_, Arc<DatabaseState>>,
) -> Result<bool, String> {
    let mut conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection to disconnect.".into());
    }

    let conn: Connection = match conn_guard.take() {
        Some(conn) => conn,
        None => return Err("No active database connection to disconnect.".into()),
    };

    conn.disconnect();

    Ok(true)
}

#[command]
pub async fn generate_load_data_sql(load: GenerateLoadData) -> Result<String, String> {
    if load.file_path.is_empty() {
        return Err("File path is empty".into());
    }

    if load.db_driver == SQLITE {
        return Err("SQLite is not supported for this operation.".into());
    }

    let file: File =
        File::open(&load.file_path).map_err(|e| format!("Failed to open file: {e}"))?;
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_reader(file);

    let first_line: String = read_first_line(&load.file_path).unwrap();
    let separator: char = find_separator(&first_line)?;
    let final_columns_name: Vec<String> = get_formated_column_names(
        first_line
            .split(separator)
            .map(|s| s.replace("\"", ""))
            .collect(),
    );

    let mut columns_size_map: HashMap<&str, usize> = HashMap::new();
    for header in &final_columns_name {
        columns_size_map.insert(header, 0);
    }

    for record in reader.records() {
        let record: StringRecord = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        for (i, value) in record.iter().enumerate() {
            let value: String = value.trim().into();
            let max_length: &mut usize = columns_size_map
                .get_mut(final_columns_name[i].as_str())
                .unwrap();
            if value.len() > *max_length {
                *max_length = value.len() + 1;
            }
        }
    }

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

    sql.push_str(build_load_data(load, separator, final_columns_name)?.as_str());

    Ok(sql)
}

#[command]
pub async fn execute_sql(
    state: State<'_, Arc<DatabaseState>>,
    sql: String,
) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err("No active database connection.".into());
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
pub async fn is_connected(state: State<'_, Arc<DatabaseState>>) -> Result<String, bool> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Ok(String::new());
    }

    let connection: &Connection = conn_guard.as_ref().unwrap();
    let db_config: &DbConfig = connection.get_db_config();

    // Return the database configuration as JSON string
    match serde_json::to_string(db_config) {
        Ok(json) => Ok(json),
        Err(_) => Err(false),
    }
}
