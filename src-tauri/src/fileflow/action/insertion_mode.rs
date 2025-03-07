use crate::fileflow::database::connection::Connection;
use crate::fileflow::utils::fileflowlib::{escaped_values, sanitize_value};
use crate::fileflow::utils::sql::{
    batch_insert, create_and_copy_final_table, drop_table_if_exists, execute_query,
    get_create_statement, get_insert_into_statement,
};
use csv::{Reader, StringRecord};
use std::collections::HashMap;
use std::fs::File;

/// Insert data into the database using the optimized table creation and insertion method
pub async fn optimized_insert(
    connection: &Connection,
    reader: &mut Reader<File>,
    final_columns_name: &[String],
    final_table_name: &str,
    db_driver: &str,
) -> Result<u64, String> {
    const MAX_BATCH_SIZE: usize = 4000;
    let temporary_table_name: &str = &format!("{final_table_name}_temporary");

    // Drop existing tables
    drop_existing_tables(
        connection,
        &[temporary_table_name, final_table_name],
        db_driver,
    )
    .await?;

    // Create the temporary table
    let create_temp_table_query: String =
        get_create_statement(db_driver, temporary_table_name, final_columns_name)?;
    execute_query(
        connection,
        &create_temp_table_query,
        "Failed to create temporary table",
    )
    .await?;

    // Initialize variables
    let mut batch: Vec<String> = Vec::with_capacity(MAX_BATCH_SIZE);
    let mut columns_size_map: HashMap<&str, usize> =
        initialize_columns_size_map(final_columns_name);
    let insert_query_base: String = get_insert_into_statement(
        db_driver,
        temporary_table_name,
        &final_columns_name.join(", "),
    )?;
    let mut line_count: u64 = 0;

    // Read and process records from the CSV
    for result in reader.records() {
        let record: StringRecord = result.map_err(|err| format!("CSV record error: {err}"))?;
        process_record(
            &record,
            &mut batch,
            &mut columns_size_map,
            final_columns_name,
        )?;

        if batch.len() >= MAX_BATCH_SIZE {
            line_count += insert_batch(connection, &insert_query_base, &batch).await?;
            batch.clear();
        }
    }

    // Insert remaining records
    if !batch.is_empty() {
        line_count += insert_batch(connection, &insert_query_base, &batch).await?;
    }

    // Create final table and copy data
    create_and_copy_final_table(
        connection,
        db_driver,
        final_table_name,
        temporary_table_name,
        &columns_size_map,
        final_columns_name,
    )
    .await?;

    // Drop the temporary table
    drop_table_if_exists(connection, db_driver, temporary_table_name).await?;

    Ok(line_count)
}

/// Drop a list of tables if they exist
async fn drop_existing_tables(
    connection: &Connection,
    table_names: &[&str],
    db_driver: &str,
) -> Result<(), String> {
    for table_name in table_names {
        drop_table_if_exists(connection, db_driver, table_name).await?;
    }
    Ok(())
}

/// Initialize the column size map with default sizes
fn initialize_columns_size_map(final_columns_name: &[String]) -> HashMap<&str, usize> {
    final_columns_name
        .iter()
        .map(|col| (col.as_str(), 0))
        .collect()
}

/// Sanitize and process a CSV record, updating batch and column size map
fn process_record(
    record: &StringRecord,
    batch: &mut Vec<String>,
    columns_size_map: &mut HashMap<&str, usize>,
    final_columns_name: &[String],
) -> Result<(), String> {
    let mut values: Vec<String> = Vec::with_capacity(record.len());

    for (i, value) in record.iter().enumerate() {
        let sanitized_value: String = sanitize_value(value);
        let max_length: &mut usize = columns_size_map
            .get_mut(final_columns_name[i].as_str())
            .ok_or("Column name mismatch")?;
        *max_length = (*max_length).max(sanitized_value.len() + 1);
        values.push(format!("'{sanitized_value}'"));
    }

    batch.push(format!("({})", values.join(", ")));
    Ok(())
}

/// Insert a batch of records into the database
async fn insert_batch(
    connection: &Connection,
    insert_query_base: &str,
    batch: &[String],
) -> Result<u64, String> {
    batch_insert(
        connection,
        insert_query_base,
        batch,
        "Failed to insert batch data",
    )
    .await
    .map(|_| batch.len() as u64)
}

/// Fast insert data the csv file into the database table
pub async fn fast_insert(
    connection: &Connection,
    reader: &mut Reader<File>,
    final_columns_name: &[String],
    final_table_name: &str,
    db_driver: &str,
) -> Result<u64, String> {
    const MAX_BATCH_SIZE: usize = 4000;

    // Drop the table if it exists
    drop_table_if_exists(connection, db_driver, final_table_name).await?;

    // Create the table
    execute_query(
        connection,
        get_create_statement(db_driver, final_table_name, final_columns_name)?.as_str(),
        "Failed to create table {final_table_name}",
    )
    .await?;

    let columns: &str = &final_columns_name.join(", ");
    let mut line_count: u64 = 0;
    let mut batch: Vec<String> = Vec::with_capacity(MAX_BATCH_SIZE);

    // Prepare the insert query
    let insert_query_base: &str = &get_insert_into_statement(db_driver, final_table_name, columns)?;

    for result in reader.records() {
        let record: StringRecord = result.unwrap();
        let values: String = escaped_values(record);
        batch.push(format!("({values})"));

        if batch.len() == MAX_BATCH_SIZE {
            batch_insert(
                connection,
                insert_query_base,
                &batch,
                "Failed to insert batch data",
            )
            .await?;
            line_count += batch.len() as u64;
            batch.clear();
        }
    }

    // Insert the remaining records if any
    batch_insert(
        connection,
        insert_query_base,
        &batch,
        "Failed to insert batch data",
    )
    .await?;
    line_count += batch.len() as u64;

    Ok(line_count)
}
