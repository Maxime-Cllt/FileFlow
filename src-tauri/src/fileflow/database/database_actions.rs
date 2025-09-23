use crate::fileflow::enums::database_engine::DatabaseEngine;
use crate::fileflow::enums::query_result::QueryRows;
use crate::fileflow::stuct::database_connection::DatabaseConnection;
use crate::fileflow::stuct::download_config::DownloadConfig;
use crate::fileflow::stuct::sql_builder::SqlBuilder;
use csv::{Writer, WriterBuilder};
use sqlx::{Column, Row};
use std::collections::HashMap;
use std::fs::File;

/// Exports a table’s data into a CSV file. It uses offset/LIMIT pagination to retrieve data in batches
pub async fn export_table(
    connection: &DatabaseConnection,
    download_config: &DownloadConfig,
    table_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    const LIMIT: u16 = 5_000;
    let mut offset: u32 = 0;
    let mut header_written: bool = false;
    let file_path: String = format!("{}/{table_name}_export.csv", download_config.location);

    let mut wtr: Writer<File> = WriterBuilder::new()
        .delimiter(download_config.separator.as_u8())
        .from_path(&file_path)
        .expect("Failed to create CSV writer");

    let base_sql: String = format!("SELECT * FROM {table_name}");

    loop {
        let sql_query: String = format!("{base_sql} LIMIT {LIMIT} OFFSET {offset}");
        let query_result: QueryRows = connection.query_many_with_result(&sql_query).await?;

        let (columns, rows): (Vec<String>, Vec<Vec<String>>) = match query_result {
            QueryRows::MySQL(rows) => {
                if rows.is_empty() {
                    break;
                }
                let cols = rows[0]
                    .columns()
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect::<Vec<_>>();
                let data = rows
                    .into_iter()
                    .map(|row| {
                        row.columns()
                            .iter()
                            .map(|col| {
                                row.try_get::<Option<String>, _>(col.name())
                                    .unwrap_or(None)
                                    .unwrap_or_default()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();
                (cols, data)
            }
            QueryRows::Postgres(rows) => {
                if rows.is_empty() {
                    break;
                }
                let cols = rows[0]
                    .columns()
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect::<Vec<_>>();
                let data = rows
                    .into_iter()
                    .map(|row| {
                        row.columns()
                            .iter()
                            .map(|col| {
                                row.try_get::<Option<String>, _>(col.name())
                                    .unwrap_or(None)
                                    .unwrap_or_default()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();
                (cols, data)
            }
            QueryRows::SQLite(rows) => {
                if rows.is_empty() {
                    break;
                }
                let cols = rows[0]
                    .columns()
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect::<Vec<_>>();
                let data = rows
                    .into_iter()
                    .map(|row| {
                        row.columns()
                            .iter()
                            .map(|col| {
                                row.try_get::<Option<String>, _>(col.name())
                                    .unwrap_or(None)
                                    .unwrap_or_default()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();
                (cols, data)
            }
        };

        if !header_written {
            wtr.write_record(&columns).expect("Failed to write columns");
            header_written = true;
        }

        for record in rows.iter() {
            wtr.write_record(record)
                .expect("Failed to write record to CSV");
        }

        offset += u32::from(LIMIT);
    }

    wtr.flush().expect("Failed to flush CSV writer");
    Ok(())
}

/// Helper function to drop a table if it exists
pub async fn drop_table_if_exists(
    connection: &DatabaseConnection,
    db_driver: &DatabaseEngine,
    table_name: &str,
) -> Result<(), String> {
    let drop_query: String = SqlBuilder::build_drop_statement(db_driver, table_name)?;
    execute_query(
        connection,
        &drop_query,
        &format!("Failed to drop table '{table_name}'"),
    )
    .await
}

/// Helper function to execute a query and handle errors
pub async fn execute_query(
    connection: &DatabaseConnection,
    query: &str,
    context: &str,
) -> Result<(), String> {
    connection
        .query(query)
        .await
        .map_err(|err| format!("{context}: {err}"))
}

/// Helper function to batch-insert records into a table
pub async fn batch_insert(
    connection: &DatabaseConnection,
    insert_query_base: &str,
    batch: &[String],
    context: &str,
) -> Result<(), String> {
    if batch.is_empty() {
        return Ok(());
    }
    let insert_query: String = format!("{insert_query_base}{}", batch.join(", "));
    execute_query(connection, &insert_query, context).await
}

/// Create the final table and copy data from the temporary table
pub async fn create_and_copy_final_table(
    connection: &DatabaseConnection,
    db_driver: &DatabaseEngine,
    final_table_name: &str,
    temporary_table_name: &str,
    columns_size_map: &HashMap<&str, usize>,
    final_columns_name: &[String],
) -> Result<(), String> {
    let create_final_table_query: String = SqlBuilder::build_create_with_fixed_size(
        db_driver,
        final_table_name,
        columns_size_map,
        final_columns_name,
    )?;

    execute_query(
        connection,
        &create_final_table_query,
        "Failed to create final table",
    )
    .await?;

    let copy_data_query: String =
        SqlBuilder::build_copy_table(db_driver, temporary_table_name, final_table_name)?;

    execute_query(
        connection,
        &copy_data_query,
        "Failed to copy data to final table",
    )
    .await?;

    Ok(())
}

/// Drop a list of tables if they exist
pub async fn drop_existing_tables(
    connection: &DatabaseConnection,
    table_names: &[&str],
    db_driver: &DatabaseEngine,
) -> Result<(), String> {
    for table_name in table_names.iter() {
        if let Err(e) = drop_table_if_exists(connection, db_driver, table_name).await {
            eprintln!("Error: {e}");
        }
    }
    Ok(())
}
