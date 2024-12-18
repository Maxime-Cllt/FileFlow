use crate::fileflow::database::connection::Connection;
use crate::fileflow::utils::sql::{
    batch_insert, drop_table_if_exists, escaped_values, execute_query, get_create_statement,
    get_insert_into_statement,
};
use csv::{Reader, StringRecord};
use std::fs::File;

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

    execute_query(
        connection,
        get_create_statement(db_driver, final_table_name, final_columns_name)?.as_str(),
        "Failed to create table {final_table_name}",
    )
    .await?;

    let columns: &str = &final_columns_name.join(", ");
    let mut line_count: u64 = 0;
    let mut batch: Vec<String> = Vec::with_capacity(MAX_BATCH_SIZE);

    let insert_query_base: &str = &get_insert_into_statement(db_driver, final_table_name, columns)?;

    for result in reader.records() {
        let record: StringRecord = result.unwrap();
        let values: String = escaped_values(record);
        batch.push(format!("({values})"));

        if batch.len() == MAX_BATCH_SIZE {
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

    // Insert the remaining records if any
    batch_insert(
        connection,
        &insert_query_base,
        &batch,
        "Failed to insert batch data",
    )
    .await?;
    line_count += batch.len() as u64;

    Ok(line_count)
}
