use std::fs::File;
use csv::{Reader, StringRecord};
use crate::{DatabaseConnection};

use crate::fileflow::fileflow::{get_drop_statement, get_create_statement, get_insert_into_statement};

pub(crate) async fn fast_insert(
    conn: &DatabaseConnection,
    reader: &mut Reader<File>,
    snake_case_headers: &Vec<String>,
    final_table_name: &str,
    db_driver: &str,
) -> Result<u64, String> {

    // Drop the table if it exists
    if let Err(err) = conn.query(&get_drop_statement(db_driver, final_table_name)?).await {
        return Err(format!("Failed to drop final table: {}", err));
    }

    // Create the table
    if let Err(err) = conn.query(&get_create_statement(db_driver, final_table_name, snake_case_headers)?).await {
        return Err(format!("Failed to create table: {}", err));
    }

    let columns: &str = &snake_case_headers.join(", ");
    let mut line_count: u64 = 0;
    let max_batch_size: u16 = 5000;
    let mut batch: Vec<String> = Vec::with_capacity(max_batch_size as usize);

    let insert_query_base: &str = &get_insert_into_statement(db_driver, final_table_name, &columns)?;

    for result in reader.records() {
        let record: StringRecord = result.unwrap();

        let values: Vec<String> = record.iter()
            .map(|v| format!("'{}'", v.trim().replace("'", "''")))
            .collect();

        batch.push(format!("({})", values.join(", ")));

        if batch.len() == max_batch_size as usize {
            let insert_query: String = format!("{}{}", &insert_query_base, batch.join(", "));
            if let Err(err) = conn.query(&insert_query).await {
                return Err(format!("Failed to insert data: {}", err));
            }
            line_count += batch.len() as u64;
            batch.clear();
        }
    }

    // Insert the remaining records if any
    if !batch.is_empty() {
        let insert_query: &str = &format!("{}{}", &insert_query_base, batch.join(", "));
        if let Err(err) = conn.query(&insert_query).await {
            return Err(format!("Failed to insert data: {}", err));
        }
        line_count += batch.len() as u64;
    }

    Ok(line_count)
}