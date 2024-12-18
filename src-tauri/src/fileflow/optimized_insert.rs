use crate::fileflow::database::connection::Connection;
use crate::fileflow::utils::sql::{
    batch_insert, drop_table_if_exists, execute_query, get_copy_temp_to_final_table,
    get_create_statement, get_create_statement_with_fixed_size, get_insert_into_statement,
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

    // Define the temporary table name
    let temporary_table_name: String = format!("{final_table_name}_temporary");

    // Drop both the temporary and final tables if they exist
    for table_name in &[&temporary_table_name, final_table_name] {
        drop_table_if_exists(connection, db_driver, table_name).await?;
    }

    // Create the temporary table
    let create_temp_table_query: String =
        get_create_statement(db_driver, &temporary_table_name, final_columns_name)?;
    execute_query(
        connection,
        &create_temp_table_query,
        "Failed to create temporary table",
    )
    .await?;

    // Initialize batch insertion variables
    let mut batch: Vec<String> = Vec::with_capacity(MAX_BATCH_SIZE);
    let mut columns_size_map: HashMap<&str, usize> = final_columns_name
        .iter()
        .map(|col| (col.as_str(), 0))
        .collect();
    let columns: String = final_columns_name.join(", ");
    let insert_query_base: String =
        get_insert_into_statement(db_driver, &temporary_table_name, &columns)?;
    let mut line_count: u64 = 0;

    // Read and process records
    for result in reader.records() {
        let record: StringRecord = result.map_err(|err| format!("CSV record error: {err}"))?;
        let mut values: Vec<String> = Vec::with_capacity(record.len());

        for (i, value) in record.iter().enumerate() {
            let sanitized_value: String = value.trim().replace('\'', "''").replace('\\', "\\\\");
            let max_length: &mut usize = columns_size_map
                .get_mut(final_columns_name[i].as_str())
                .expect("Column name mismatch");
            *max_length = (*max_length).max(sanitized_value.len() + 1);
            values.push(format!("'{sanitized_value}'"));
        }

        batch.push(format!("({})", values.join(", ")));

        // Insert batch when it reaches the maximum size
        if batch.len() >= MAX_BATCH_SIZE {
            batch_insert(
                connection,
                &insert_query_base,
                &batch,
                "Failed to insert batch data",
            )
            .await?;
            line_count += batch.len() as u64;
            batch.clear();
        }
    }

    // Insert any remaining records in the batch
    batch_insert(
        connection,
        &insert_query_base,
        &batch,
        "Failed to insert remaining batch",
    )
    .await?;
    line_count += batch.len() as u64;

    // Create the final table with fixed-size columns
    execute_query(
        connection,
        get_create_statement_with_fixed_size(
            db_driver,
            final_table_name,
            &columns_size_map,
            final_columns_name,
        )?
        .as_str(),
        "Failed to create final table",
    )
    .await?;

    // Copy data from the temporary table to the final table
    execute_query(
        connection,
        get_copy_temp_to_final_table(db_driver, &temporary_table_name, final_table_name)?.as_str(),
        "Failed to copy data to final table",
    )
    .await?;

    // Drop the temporary table
    drop_table_if_exists(connection, db_driver, &temporary_table_name).await?;

    Ok(line_count)
}
