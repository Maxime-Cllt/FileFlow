use std::collections::HashMap;
use std::fs::File;
use csv::{Reader, StringRecord};
use crate::fileflow::database_connection::DatabaseConnection;
use crate::fileflow::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};

use crate::fileflow::fileflow::{get_drop_statement, get_create_statement, get_insert_into_statement, get_create_statement_with_fixed_size};

pub async fn optimized_insert(
    connection: &DatabaseConnection,
    reader: &mut Reader<File>,
    snake_case_headers: &Vec<String>,
    final_table_name: &str,
    db_driver: &str,
) -> Result<u64, String> {
    let temporary_table_name: &str = &format!("{}_temporary", final_table_name);

    for table_name in &[&temporary_table_name, final_table_name] {
        let drop_table_query: String = get_drop_statement(db_driver, table_name)?;
        if let Err(err) = connection.query(&drop_table_query).await {
            return Err(format!("Failed to drop table '{}': {}", table_name, err));
        }
    }

    let create_table_query: &str = &get_create_statement(db_driver, &temporary_table_name, &snake_case_headers)?;

    if let Err(err) = connection.query(&create_table_query).await {
        return Err(format!("Failed to create temporary table: {}", err));
    }

    let max_batch_size: usize = 4000;
    let mut batch: Vec<String> = Vec::with_capacity(max_batch_size);
    let mut map_max_length: HashMap<&str, usize> = snake_case_headers.iter().map(|h| (h.as_str(), 0)).collect();
    let columns: String = snake_case_headers.join(", ");
    let insert_query_base: String = get_insert_into_statement(db_driver, &temporary_table_name, &columns)?;
    let mut line_count: u64 = 0;

    for result in reader.records() {
        let record: StringRecord = result.unwrap();
        let mut values: Vec<String> = Vec::with_capacity(record.len());

        for (i, value) in record.iter().enumerate() {
            let value: String = value.trim().replace("'", "''");
            let max_length: &mut usize = map_max_length.get_mut(snake_case_headers[i].as_str()).unwrap();
            if value.len() > *max_length {
                *max_length = value.len() + 1;
            }
            values.push(format!("'{}'", value));
        }

        let values: Vec<String> = values.iter()
            .map(|v| v.replace("\\", "\\\\"))
            .collect();
        batch.push(format!("({})", values.join(", ")));

        if batch.len() >= max_batch_size {
            let insert_query: String = format!("{}{}", insert_query_base, batch.join(", "));
            if let Err(err) = connection.query(&insert_query).await {
                return Err(format!("Failed to insert batch data: {}", err));
            }
            line_count += batch.len() as u64;
            batch.clear();
        }
    }

    if !batch.is_empty() {
        let insert_query: String = format!("{}{}", insert_query_base, batch.join(", "));
        if let Err(err) = connection.query(&insert_query).await {
            return Err(format!("Failed to insert remaining batch: {}", err));
        }
        line_count += batch.len() as u64;
    }

    let create_final_table_query: &str = &get_create_statement_with_fixed_size(db_driver, final_table_name, &map_max_length, &snake_case_headers)?;

    if let Err(err) = connection.query(&create_final_table_query).await {
        return Err(format!("Failed to create final table: {}", err));
    }

    let insert_final_query: &str = &get_copy_temp_to_final_table(db_driver, &temporary_table_name, final_table_name)?;
    if let Err(err) = connection.query(&insert_final_query).await {
        return Err(format!("Failed to insert data into final table: {}", err));
    }

    let drop_temp_table_query: &str = &get_drop_statement(db_driver, &temporary_table_name)?;
    if let Err(err) = connection.query(&drop_temp_table_query).await {
        return Err(format!("Failed to drop temporary table: {}", err));
    }

    Ok(line_count)
}

fn get_copy_temp_to_final_table(driver: &str, temporary_table_name: &str, final_table_name: &str) -> Result<String, String> {
    match driver {
        SQLITE => {
            Ok(format!(
                "INSERT INTO \"{}\" SELECT * FROM \"{}\"",
                final_table_name,
                temporary_table_name
            ))
        }
        MYSQL | MARIADB => {
            Ok(format!(
                "INSERT INTO `{}` SELECT * FROM `{}`",
                final_table_name,
                temporary_table_name
            ))
        }
        POSTGRES => {
            Ok(format!(
                "INSERT INTO \"{}\" SELECT * FROM \"{}\"",
                final_table_name,
                temporary_table_name
            ))
        }
        _ => Err(format!("Unsupported database driver: {}", driver)),
    }
}