use crate::fileflow::action::actions::DatabaseState;
use crate::fileflow::database::connection::{Connection, QueryResult};
use crate::fileflow::database::database_actions::{
    batch_insert, create_and_copy_final_table, drop_existing_tables, drop_table_if_exists,
    execute_query, export_table,
};
use crate::fileflow::database::sql_builder::{
    build_create_table_sql, build_prepared_statement_sql, build_query_all_tables,
};
use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use crate::fileflow::stuct::combo_item::ComboItem;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::download_config::DownloadConfig;
use csv::{Reader, StringRecord};
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use std::time::Instant;
use tauri::{command, State};
use crate::fileflow::stuct::string_formater::StringFormatter;

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

#[command]
pub async fn get_table_list(state: State<'_, Arc<DatabaseState>>) -> Result<Value, bool> {
    let conn_guard = state.0.lock().await;

    if conn_guard.is_none() {
        return Err(false);
    }

    let connection: &Connection = match conn_guard.as_ref() {
        Some(conn) => conn,
        None => return Err(false),
    };

    let db_config: &DbConfig = connection.get_db_config();
    let sql: &str = &build_query_all_tables(&db_config.db_driver, &db_config.db_name);

    let result: QueryResult = connection
        .query_many_with_result(sql)
        .await
        .map_err(|_| false)?;

    let mut vec: Vec<ComboItem> = Vec::new();

    match result {
        QueryResult::MySQL(rows) => {
            for row in rows {
                vec.push(ComboItem {
                    value: row.get("TABLE_NAME"),
                    label: row.get("TABLE_NAME"),
                });
            }
        }
        QueryResult::Postgres(rows) => {
            for row in rows {
                vec.push(ComboItem {
                    value: row.get("table_name"),
                    label: row.get("table_name"),
                });
            }
        }
        QueryResult::SQLite(rows) => {
            for row in rows {
                vec.push(ComboItem {
                    value: row.get("name"),
                    label: row.get("name"),
                });
            }
        }
    }

    Ok(json!(vec))
}

#[command]
pub async fn download_table(
    config: DownloadConfig,
    state: State<'_, Arc<DatabaseState>>,
) -> Result<String, String> {
    let conn_guard = state.0.lock().await;

    let start: Instant = Instant::now();

    if conn_guard.is_none() {
        return Err("No active database connection.".into());
    }

    if config.table_name_list.is_empty() || config.location.is_empty() {
        return Err("Some required fields are missing.".into());
    }

    let connection: &Connection = match conn_guard.as_ref() {
        Some(conn) => conn,
        None => return Err("No active database connection.".into()),
    };

    let mut exported_table: u32 = 0;
    for table_name in config.table_name_list.iter() {
        if let Err(err) = export_table(connection, &config, table_name).await {
            println!("{err}");
            continue;
        }
        exported_table += 1;
    }

    let response: String = if exported_table == 0 {
        "No tables were exported.".into()
    } else {
        format!(
            "Exported {exported_table} out of {} tables successfully in {:?} seconds.",
            config.table_name_list.len(),
            start.elapsed()
        )
    };

    Ok(response)
}

/// Fast insert data the csv file into the database table
pub async fn fast_insert(
    connection: &Connection,
    reader: &mut Reader<File>,
    final_columns_name: &[String],
    final_table_name: &str,
    db_driver: &DatabaseEngine,
) -> Result<u32, String> {
    // Drop the table if it exists
    if let Err(err) = drop_table_if_exists(connection, db_driver, final_table_name).await {
        eprintln!("Error: {err}");
        return Err(err);
    }

    let build_create_table_statement: String = build_create_table_sql(db_driver, final_table_name, final_columns_name);
    
    println!("Create table query: {build_create_table_statement}");

    // Create the table
    if let Err(err) = execute_query(
        connection,
        &build_create_table_statement,
        "Failed to create table",
    )
    .await
    {
        eprintln!("Error: {err}");
        return Err(err);
    }

    const MAX_BATCH_SIZE: usize = 5_000;
    let mut line_count: u32 = 0;
    let mut batch: Vec<String> = Vec::with_capacity(MAX_BATCH_SIZE);

    // Prepare the insert query
    let insert_query_base: &str =
        &build_prepared_statement_sql(db_driver, final_table_name, &final_columns_name);
    
    println!("Insert query base: {insert_query_base}");

    for result in reader.records() {
        let values: String = match result {
            Ok(record) => StringFormatter::escape_record(record),
            Err(_) => continue,
        };

        batch.push(format!("({values})"));

        if batch.len() >= MAX_BATCH_SIZE {
            line_count += insert_batch(connection, insert_query_base, &batch).await;
            batch.clear();
        }
    }

    // Insert the remaining records if any
    line_count += insert_batch(connection, insert_query_base, &batch).await;

    Ok(line_count)
}

/// Insert data into the database using the optimized table creation and insertion method
pub async fn optimized_insert(
    connection: &Connection,
    reader: &mut Reader<File>,
    final_columns_name: &[String],
    final_table_name: &str,
    db_driver: &DatabaseEngine,
) -> Result<u32, String> {
    // Drop existing tables
    let temporary_table_name: String = format!("{final_table_name}_temporary");
    drop_existing_tables(
        connection,
        &[&temporary_table_name, final_table_name],
        db_driver,
    )
    .await
    .expect("Failed to drop existing tables");

    // Create the temporary table
    let create_temp_table_query: String =
        build_create_table_sql(db_driver, &temporary_table_name, final_columns_name);

    execute_query(
        connection,
        &create_temp_table_query,
        "Failed to create temporary table",
    )
    .await
    .expect("Failed to create temporary table query");

    // Initialize variables
    const MAX_BATCH_SIZE: usize = 5_000;
    let insert_query_base: String =
        build_prepared_statement_sql(db_driver, &temporary_table_name, &final_columns_name);

    let mut columns_size_map: HashMap<&str, usize> = final_columns_name
        .iter()
        .map(|col| (col.as_str(), 0))
        .collect(); // Initialize column size map for each column -> 0 (id,size)
    let mut line_count: u32 = 0;
    let mut batch: Vec<String> = Vec::with_capacity(MAX_BATCH_SIZE);

    for result in reader.records() {
        let record: StringRecord = match result {
            Ok(record) => record,
            Err(_) => continue,
        };

        let mut values: Vec<String> = Vec::with_capacity(record.len());

        for (i, value) in record.iter().enumerate() {
            let sanitized_value: String = StringFormatter::sanitize_value(value);
            let max_length: &mut usize = columns_size_map
                .get_mut(final_columns_name[i].as_str())
                .ok_or("Column name mismatch")
                .expect("Column name mismatch");
            *max_length = (*max_length).max(sanitized_value.len() + 1);
            values.push(format!("'{sanitized_value}'"));
        }

        batch.push(format!("({})", values.join(", ")));

        if batch.len() >= MAX_BATCH_SIZE {
            line_count += insert_batch(connection, &insert_query_base, &batch).await;
            batch.clear();
        }
    }

    // Insert remaining records
    if !batch.is_empty() {
        line_count += insert_batch(connection, &insert_query_base, &batch).await;
    }

    // Create final table and copy data
    create_and_copy_final_table(
        connection,
        db_driver,
        final_table_name,
        &temporary_table_name,
        &columns_size_map,
        final_columns_name,
    )
    .await?;

    drop_table_if_exists(connection, db_driver, &temporary_table_name).await?; // Drop the temporary table

    Ok(line_count)
}

/// Insert a batch of records into the database
async fn insert_batch(connection: &Connection, insert_query_base: &str, batch: &[String]) -> u32 {
    match batch_insert(
        connection,
        insert_query_base,
        batch,
        "Failed to insert batch data",
    )
    .await
    {
        Ok(_) => u32::try_from(batch.len()).unwrap_or(5_000),
        Err(err) => {
            eprintln!("Error inserting batch: {err}");
            0
        }
    }
}
