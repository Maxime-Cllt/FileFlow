use crate::fileflow::action::actions::DatabaseState;
use crate::fileflow::database::connection::{Connection, QueryResult};
use crate::fileflow::stuct::combo_item::ComboItem;
use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::stuct::download_config::DownloadConfig;
use crate::fileflow::utils::sql::{export_table, get_all_tables_query};
use serde_json::{json, Value};
use sqlx::Row;
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

    let sql: &str = &get_all_tables_query(&db_config.db_driver, &db_config.db_name)
        .expect("Failed to get all tables query");

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

    if config.table_name.is_empty() || config.location.is_empty() {
        return Err("Some required fields are missing.".into());
    }

    let connection: &Connection = match conn_guard.as_ref() {
        Some(conn) => conn,
        None => return Err("No active database connection.".into()),
    };

    if let Err(err) = export_table(connection, config).await {
        return Err(format!("Failed to export table: {err}"));
    }

    Ok(format!(
        "Table downloaded successfully in {:?} seconds.",
        start.elapsed()
    ))
}
