use crate::fileflow::database::connection::{Connection, QueryResult};
use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use crate::fileflow::enumeration::separator::SeparatorType;
use crate::fileflow::stuct::download_config::DownloadConfig;
use csv::{Writer, WriterBuilder};
use sqlx::{Column, Row};
use std::collections::HashMap;
use std::fs::File;

/// This function is used to generate the DROP TABLE statement for different database drivers.
pub fn get_drop_statement(
    db_driver: &DatabaseEngine,
    final_table_name: &str,
) -> Result<String, String> {
    match db_driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
            Ok(format!("DROP TABLE IF EXISTS \"{final_table_name}\""))
        }
        DatabaseEngine::MariaDB | DatabaseEngine::MySQL => {
            Ok(format!("DROP TABLE IF EXISTS `{final_table_name}`"))
        }
    }
}

/// This function is used to generate the INSERT INTO statement for different database drivers.
pub fn get_insert_into_statement(
    db_driver: &DatabaseEngine,
    final_table_name: &str,
    columns: &str,
) -> Result<String, String> {
    let quote: char = match db_driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => '\"',
        DatabaseEngine::MariaDB | DatabaseEngine::MySQL => '`',
    };

    Ok(format!(
        "INSERT INTO {quote}{final_table_name}{quote} ({columns}) VALUES "
    ))
}

/// This function is used to generate the COPY statement for different database drivers.
pub fn get_copy_temp_to_final_table(
    db_driver: &DatabaseEngine,
    temporary_table_name: &str,
    final_table_name: &str,
) -> Result<String, String> {
    let quote: char = match db_driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => '\"',
        DatabaseEngine::MySQL | DatabaseEngine::MariaDB => '`',
    };
    Ok(format!(
        "INSERT INTO {quote}{final_table_name}{quote} SELECT * FROM {quote}{temporary_table_name}{quote}"
    ))
}

/// This function is used to generate the CREATE TABLE statement with fixed size for each column for different database drivers.
pub fn get_create_statement_with_fixed_size(
    driver: &DatabaseEngine,
    final_table_name: &str,
    map_column_max_length: &HashMap<&str, usize>,
    snake_case_headers: &[String],
) -> Result<String, String> {
    // Constants
    const MAX_VARCHAR_LENGTH: usize = 255;
    const TEXT_TYPE: &str = "TEXT";
    const VARCHAR_TYPE: &str = "VARCHAR";

    // Determine database-specific formatting
    let quote = match driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => '"',
        DatabaseEngine::MySQL | DatabaseEngine::MariaDB => '`',
    };

    let mut columns: Vec<String> = Vec::with_capacity(snake_case_headers.len());
    let mut total_length: usize = 0;

    // Build column definitions
    for header in snake_case_headers {
        let max_length = map_column_max_length
            .get(header.as_str())
            .copied()
            .unwrap_or(MAX_VARCHAR_LENGTH);

        // Determine column type
        let type_str = if max_length <= MAX_VARCHAR_LENGTH {
            format!("{}({})", VARCHAR_TYPE, max_length)
        } else {
            TEXT_TYPE.to_string()
        };

        // Format column definition
        let column = format!("{0}{1}{0} {2}", quote, header, type_str);
        total_length += column.len();
        columns.push(column);
    }

    // Pre-calculate final string capacity
    let table_quoted: String = format!("{0}{1}{0}", quote, final_table_name);
    let mut result: String = String::with_capacity(
        15 + // "CREATE TABLE  ();"
            table_quoted.len() +
            total_length +
            columns.len() * 2, // ", " separators
    );

    // Build the final SQL statement
    result.push_str("CREATE TABLE ");
    result.push_str(&table_quoted);
    result.push_str(" (");
    result.push_str(&columns.join(", "));
    result.push_str(");");

    Ok(result)
}

/// This function is used to generate the CREATE TABLE statement for different database drivers.
pub fn get_create_statement(
    driver: &DatabaseEngine,
    final_table_name: &str,
    snake_case_headers: &[String],
) -> Result<String, String> {
    match driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => Ok(format!(
            "CREATE TABLE \"{final_table_name}\" ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("{h} TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        DatabaseEngine::MariaDB | DatabaseEngine::MySQL => Ok(format!(
            "CREATE TABLE `{final_table_name}` ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("{h} TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        )),
    }
}

/// Helper function to drop a table if it exists
pub async fn drop_table_if_exists(
    connection: &Connection,
    db_driver: &DatabaseEngine,
    table_name: &str,
) -> Result<(), String> {
    let drop_query: &str = &get_drop_statement(db_driver, table_name)?;
    execute_query(
        connection,
        drop_query,
        &format!("Failed to drop table '{table_name}'"),
    )
    .await
}

/// Helper function to execute a query and handle errors
pub async fn execute_query(
    connection: &Connection,
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
    connection: &Connection,
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
    connection: &Connection,
    db_driver: &DatabaseEngine,
    final_table_name: &str,
    temporary_table_name: &str,
    columns_size_map: &HashMap<&str, usize>,
    final_columns_name: &[String],
) -> Result<(), String> {
    let create_final_table_query: String = get_create_statement_with_fixed_size(
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
        get_copy_temp_to_final_table(db_driver, temporary_table_name, final_table_name)?;

    execute_query(
        connection,
        &copy_data_query,
        "Failed to copy data to final table",
    )
    .await?;

    Ok(())
}

/// Get the query to fetch all tables from the database for different drivers
pub fn build_query_all_tables(driver: &DatabaseEngine, schema: &str) -> String {
    let query: String = match driver {
        &DatabaseEngine::MySQL | &DatabaseEngine::MariaDB => format!(
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = '{schema}';"
        ),
        &DatabaseEngine::Postgres => {
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'public';".into()
        }
        &DatabaseEngine::SQLite => "SELECT name FROM sqlite_master WHERE type='table';".into(),
    };
    query
}

/// Exports a table’s data into a CSV file. It uses offset/LIMIT pagination to retrieve data in batches
pub async fn export_table(
    connection: &Connection,
    download_config: &DownloadConfig,
    table_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    const LIMIT: i32 = 5000;
    let mut offset: i32 = 0;
    let mut header_written: bool = false;
    let file_path: String = format!("{}/{table_name}_export.csv", download_config.location);

    let separator: u8 = match download_config.separator {
        SeparatorType::Comma => b',',
        SeparatorType::Semicolon => b';',
        SeparatorType::Pipe => b'|',
        SeparatorType::Space => b' ',
    };

    let mut wtr: Writer<File> = WriterBuilder::new()
        .delimiter(separator)
        .from_path(&file_path)
        .expect("Failed to create CSV writer");

    let base_sql: String = format!("SELECT * FROM {table_name}");

    loop {
        let sql_query: String = format!("{base_sql} LIMIT {LIMIT} OFFSET {offset}");
        let query_result: QueryResult = connection.query_many_with_result(&sql_query).await?;

        let (columns, rows): (Vec<String>, Vec<Vec<String>>) = match query_result {
            QueryResult::MySQL(rows) => {
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
            QueryResult::Postgres(rows) => {
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
            QueryResult::SQLite(rows) => {
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

        offset += LIMIT;
    }

    wtr.flush().expect("Failed to flush CSV writer");
    Ok(())
}
