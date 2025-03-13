use crate::fileflow::database::connection::{Connection, QueryResult};
use crate::fileflow::stuct::download_config::DownloadConfig;
use crate::fileflow::utils::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use csv::{Writer, WriterBuilder};
use sqlx::{Column, Row};
use std::collections::HashMap;
use std::fs::File;

/// This function is used to generate the DROP TABLE statement for different database drivers.
pub fn get_drop_statement(db_driver: &str, final_table_name: &str) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!("DROP TABLE IF EXISTS \"{final_table_name}\"")),
        MYSQL | MARIADB => Ok(format!("DROP TABLE IF EXISTS `{final_table_name}`")),
        _ => Err("Unsupported database driver".into()),
    }
}

/// This function is used to generate the INSERT INTO statement for different database drivers.
pub fn get_insert_into_statement(
    db_driver: &str,
    final_table_name: &str,
    columns: &str,
) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!(
            "INSERT INTO \"{final_table_name}\" ({columns}) VALUES "
        )),
        MYSQL | MARIADB => Ok(format!(
            "INSERT INTO `{final_table_name}` ({columns}) VALUES "
        )),
        _ => Err("Unsupported database driver".into()),
    }
}

/// This function is used to generate the COPY statement for different database drivers.
pub fn get_copy_temp_to_final_table(
    db_driver: &str,
    temporary_table_name: &str,
    final_table_name: &str,
) -> Result<String, String> {
    match db_driver {
        SQLITE | POSTGRES => Ok(format!(
            "INSERT INTO \"{final_table_name}\" SELECT * FROM \"{temporary_table_name}\""
        )),
        MYSQL | MARIADB => Ok(format!(
            "INSERT INTO `{final_table_name}` SELECT * FROM `{temporary_table_name}`"
        )),
        _ => Err(format!("Unsupported database driver: {db_driver}")),
    }
}

/// This function is used to generate the CREATE TABLE statement with fixed size for each column for different database drivers.
pub fn get_create_statement_with_fixed_size(
    driver: &str,
    final_table_name: &str,
    map_column_max_length: &HashMap<&str, usize>,
    snake_case_headers: &[String],
) -> Result<String, String> {
    const MAX_VARCHAR_LENGTH: usize = 255;
    const TEXT_TYPE: &str = "TEXT";
    const VARCHAR_TYPE: &str = "VARCHAR";

    // Start building the SQL statement based on the driver
    let mut create_table_sql: String = match driver {
        SQLITE | POSTGRES => format!("CREATE TABLE \"{final_table_name}\" ("),
        MYSQL | MARIADB => format!("CREATE TABLE `{final_table_name}` ("),
        _ => return Err("Unsupported database driver".into()),
    };

    for header in snake_case_headers {
        let max_length: usize = if let Some(&length) = map_column_max_length.get(header.as_str()) {
            length
        } else {
            MAX_VARCHAR_LENGTH
        };

        let column_type: String = if max_length <= MAX_VARCHAR_LENGTH {
            format!("{VARCHAR_TYPE}({max_length})")
        } else {
            String::from(TEXT_TYPE)
        };

        // Quote column names as required by each driver
        let quoted_column: String = match driver {
            SQLITE | POSTGRES => format!("\"{header}\""),
            MYSQL | MARIADB => format!("`{header}`"),
            _ => return Err("Unsupported database driver".into()),
        };

        // Append the column definition to the SQL statement
        create_table_sql.push_str(&format!("{quoted_column} {column_type}, "));
    }

    // Remove the trailing comma and space, then close the SQL statement
    if create_table_sql.ends_with(", ") {
        create_table_sql.pop();
        create_table_sql.pop();
    }
    create_table_sql.push_str(");");

    Ok(create_table_sql)
}

/// This function is used to generate the CREATE TABLE statement for different database drivers.
pub fn get_create_statement(
    driver: &str,
    final_table_name: &str,
    snake_case_headers: &[String],
) -> Result<String, String> {
    match &driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!(
            "CREATE TABLE \"{final_table_name}\" ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("{h} TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        MYSQL | MARIADB => Ok(format!(
            "CREATE TABLE `{final_table_name}` ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("{h} TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        _ => Err("Unsupported database driver".into()),
    }
}

/// Helper function to drop a table if it exists
pub async fn drop_table_if_exists(
    connection: &Connection,
    db_driver: &str,
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
    db_driver: &str,
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
pub fn get_all_tables_query<'a>(driver: &'a str, schema: &'a str) -> Result<String, String> {
    match &driver.to_lowercase()[..] {
        MYSQL | MARIADB => Ok(format!(
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = '{schema}';"
        )),
        POSTGRES => Ok(
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'public';"
                .into(),
        ),
        SQLITE => Ok("SELECT name FROM sqlite_master WHERE type='table';".into()),
        _ => Err("Unsupported database driver".into()),
    }
}

/// Exports a table’s data into a CSV file. It uses offset/LIMIT pagination to retrieve data in batches
pub async fn export_table(
    connection: &Connection,
    download_config: DownloadConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    const LIMIT: i32 = 5000;
    let mut offset: i32 = 0;
    let mut header_written: bool = false;

    let file_path: String = format!(
        "{}/{}_export.csv",
        download_config.location, download_config.table_name
    );

    let separator: u8 = match &*download_config.separator {
        "," => b',',
        ";" => b';',
        "|" => b'|',
        _ => b',',
    };

    let mut wtr: Writer<File> = WriterBuilder::new()
        .delimiter(separator)
        .from_path(&file_path)
        .expect("Failed to create CSV writer");

    let base_sql: String = format!("SELECT * FROM {}", download_config.table_name);

    loop {
        let sql_query: String = format!("{} LIMIT {} OFFSET {}", base_sql, LIMIT, offset);
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

        for record in rows {
            wtr.write_record(&record)
                .expect("Failed to write record to CSV");
        }

        offset += LIMIT;
    }

    wtr.flush().expect("Failed to flush CSV writer");
    Ok(())
}
