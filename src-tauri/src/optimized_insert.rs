use std::collections::HashMap;
use std::fs::File;
use csv::Reader;
use crate::DatabaseConnection;

pub async fn optimized_insert(
    conn: &DatabaseConnection,
    reader: &mut Reader<File>,
    snake_case_headers: &Vec<String>,
    final_table_name: &str,
) -> Result<u64, String> {
    let temporary_table_name: String = format!("{}_temporary", final_table_name);

    for table_name in &[&temporary_table_name, final_table_name] {
        let drop_table_query = format!("DROP TABLE IF EXISTS \"{}\"", table_name);
        if let Err(err) = conn.query(&drop_table_query).await {
            return Err(format!("Failed to drop table '{}': {}", table_name, err));
        }
    }

    let mut map_create_temp_table: HashMap<&str, &str> = HashMap::new();
    map_create_temp_table.insert("postgres", "CREATE TABLE ");
    map_create_temp_table.insert("mysql", "CREATE TEMPORARY TABLE ");
    map_create_temp_table.insert("mariadb", "CREATE TEMPORARY TABLE ");
    map_create_temp_table.insert("sqlite", "CREATE TEMP TABLE ");

    let create_table_query = format!(
        "{} \"{}\" ({})",
        map_create_temp_table.get(&final_table_name).unwrap(),
        temporary_table_name,
        snake_case_headers
            .iter()
            .map(|h| format!("{} TEXT", h))
            .collect::<Vec<String>>()
            .join(", ")
    );
    if let Err(err) = conn.query(&create_table_query).await {
        return Err(format!("Failed to create temporary table: {}", err));
    }

    let max_batch_size = 5000;
    let mut batch = Vec::with_capacity(max_batch_size);
    let insert_query_base = format!("INSERT INTO \"{}\" ({}) VALUES ", temporary_table_name, snake_case_headers.join(", "));
    let mut map_max_length: HashMap<&str, usize> = snake_case_headers.iter().map(|h| (h.as_str(), 0)).collect();

    let mut line_count: u64 = 0;

    for result in reader.records() {
        let record = result.unwrap();
        let mut values: Vec<String> = Vec::with_capacity(record.len());

        // Collect and escape values, track maximum length per column
        for (i, value) in record.iter().enumerate() {
            let value = value.trim().replace("'", "''");  // Escape single quotes
            let max_length = map_max_length.get_mut(snake_case_headers[i].as_str()).unwrap();
            if value.len() > *max_length {
                *max_length = value.len() + 1;  // Adjust the max length for column size optimization
            }
            values.push(format!("'{}'", value));
        }

        batch.push(format!("({})", values.join(", ")));

        if batch.len() >= max_batch_size {
            // Perform batch insert when the limit is reached
            let insert_query = format!("{}{}", insert_query_base, batch.join(", "));
            if let Err(err) = conn.query(&insert_query).await {
                return Err(format!("Failed to insert batch data: {}", err));
            }
            line_count += batch.len() as u64;
            batch.clear();
        }
    }

    // Insert remaining records
    if !batch.is_empty() {
        let insert_query = format!("{}{}", insert_query_base, batch.join(", "));
        if let Err(err) = conn.query(&insert_query).await {
            return Err(format!("Failed to insert remaining batch: {}", err));
        }
        line_count += batch.len() as u64;
    }

    // Create final table with optimized column sizes
    let create_final_table_query = format!(
        "CREATE TABLE \"{}\" ({})",
        final_table_name,
        snake_case_headers
            .iter()
            .map(|header| {
                let max_length = map_max_length.get(header.as_str()).unwrap();
                format!("\"{}\" VARCHAR({})", header, max_length)
            })
            .collect::<Vec<String>>()
            .join(", ")
    );
    if let Err(err) = conn.query(&create_final_table_query).await {
        return Err(format!("Failed to create final table: {}", err));
    }

    // Insert data from temporary table to final table
    let insert_final_query = format!(
        "INSERT INTO \"{}\" SELECT * FROM \"{}\"",
        final_table_name, temporary_table_name
    );
    if let Err(err) = conn.query(&insert_final_query).await {
        return Err(format!("Failed to insert data into final table: {}", err));
    }

    let drop_temp_table_query = format!("DROP TABLE IF EXISTS \"{}\"", temporary_table_name);
    if let Err(err) = conn.query(&drop_temp_table_query).await {
        return Err(format!("Failed to drop temporary table: {}", err));
    }

    Ok(line_count)
}
