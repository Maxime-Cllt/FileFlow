use crate::fileflow::database::connection::Connection;
use crate::fileflow::fileflowlib::{
    get_create_statement, get_drop_statement, get_insert_into_statement,
};
use csv::{Reader, StringRecord};
use std::fs::File;

pub async fn fast_insert(
    connection: &Connection,
    reader: &mut Reader<File>,
    final_columns_name: &[String],
    final_table_name: &str,
    db_driver: &str,
) -> Result<u64, String> {
    // Drop the table if it exists
    if let Err(err) = connection
        .query(&get_drop_statement(db_driver, final_table_name)?)
        .await
    {
        return Err(format!("Failed to drop final table: {err}"));
    }

    // Create the table
    if let Err(err) = connection
        .query(&get_create_statement(
            db_driver,
            final_table_name,
            final_columns_name,
        )?)
        .await
    {
        return Err(format!("Failed to create table: {err}"));
    }

    let columns: &str = &final_columns_name.join(", ");
    let mut line_count: u64 = 0;
    let max_batch_size: u16 = 4000;
    let mut batch: Vec<String> = Vec::with_capacity(max_batch_size as usize);

    let insert_query_base: &str = &get_insert_into_statement(db_driver, final_table_name, columns)?;

    for result in reader.records() {
        let record: StringRecord = result.unwrap();

        let values: Vec<String> = record
            .iter()
            .map(|v| format!("'{}'", v.trim().replace("'\''", "''")))
            .collect();

        let values: Vec<String> = values.iter().map(|v| v.replace("'\\'", "\\\\")).collect();
        batch.push(format!("({})", values.join(", ")));

        if batch.len() == max_batch_size as usize {
            let insert_query: String = format!("{}{}", &insert_query_base, batch.join(", "));
            if let Err(err) = connection.query(&insert_query).await {
                return Err(format!("Failed to insert data: {err}"));
            }
            line_count += batch.len() as u64;
            batch.clear();
        }
    }

    // Insert the remaining records if any
    if !batch.is_empty() {
        let insert_query: &str = &format!("{}{}", &insert_query_base, batch.join(", "));
        if let Err(err) = connection.query(insert_query).await {
            return Err(format!("Failed to insert data: {err}"));
        }
        line_count += batch.len() as u64;
    }

    Ok(line_count)
}
