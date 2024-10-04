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

    // Drop the existing table if it exists
    let drop_table_query = format!("DROP TABLE IF EXISTS \"{}\"", temporary_table_name);  // Ensure table name is quoted
    match conn.query(&drop_table_query).await {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to drop table: {}", err)),
    }

    let drop_final_table_query = format!("DROP TABLE IF EXISTS \"{}\"", final_table_name);
    match conn.query(&drop_final_table_query).await {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to drop final table: {}", err)),
    }

    // Create temporary table
    let mut map_create_temp_table: HashMap<&str, &str> = HashMap::new();
    map_create_temp_table.insert("postgres", "CREATE TABLE ");
    map_create_temp_table.insert("mysql", "CREATE TEMPORARY TABLE ");
    map_create_temp_table.insert("mariadb", "CREATE TEMPORARY TABLE ");
    map_create_temp_table.insert("sqlite", "CREATE TEMP TABLE ");

    let create_table_query = format!(
        "{} \"{}\" ({})",  // Ensure table name is quoted
        map_create_temp_table.get(&final_table_name).unwrap(),
        temporary_table_name,
        snake_case_headers
            .iter()
            .map(|h| format!("{} TEXT", h))
            .collect::<Vec<String>>()
            .join(", ")
    );

    match conn.query(&create_table_query).await {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to create table: {}", err)),
    }


    let mut insert_query = format!("INSERT INTO \"{}\" ({}) VALUES", temporary_table_name, snake_case_headers.join(", "));
    let mut first = true;

    let mut map_max_length: HashMap<&str, usize> = HashMap::new();
    for header in snake_case_headers.iter() {
        map_max_length.insert(header.as_str(), 0);
    }

    let mut batch_insert = String::new();
    let mut batch_size = 0;
    let mut line_count = 0;
    for result in reader.records() {
        let record = result.unwrap();
        let mut values = Vec::new();

        for (i, value) in record.iter().enumerate() {
            let value = value.trim();
            let max_length = map_max_length.get_mut(snake_case_headers[i].as_str()).unwrap();
            if value.len() > *max_length {
                *max_length = value.len() + 1;
            }
            values.push(format!("'{}'", value.replace("'", "''")));  // Escape single quotes
        }

        if first {
            first = false;
        } else {
            batch_insert.push_str(", ");
        }

        line_count += 1;
        batch_size += 1;

        if batch_size >= 1000 {
            insert_query.push_str(&batch_insert);
            match conn.query(&insert_query).await {
                Ok(_) => (),
                Err(err) => return Err(format!("Failed to insert data: {}", err)),
            }
            batch_insert.clear();
            batch_size = 0;
        }
        batch_insert.push_str(&format!("({})", values.join(", ")));
    }

    if batch_size > 0 {
        insert_query.push_str(&batch_insert);
        match conn.query(&insert_query).await {
            Ok(_) => (),
            Err(err) => return Err(format!("Failed to insert data: {}", err)),
        }
    }

    // Create final table with optimized column sizes
    let mut create_final_table_query = format!("CREATE TABLE \"{}\" (", final_table_name);
    for (i, header) in snake_case_headers.iter().enumerate() {
        let max_length = map_max_length.get(header.as_str()).unwrap();
        create_final_table_query.push_str(&format!("\"{}\" VARCHAR({})", header, max_length));
        if i < snake_case_headers.len() - 1 {
            create_final_table_query.push_str(", ");
        }
    }
    create_final_table_query.push_str(")");

    match conn.query(&create_final_table_query).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to create final table: {}", err)),
    }.expect("Failed to create final table");

    // Insert data from temporary table to final table
    let insert_final_query = format!("INSERT INTO \"{}\" SELECT * FROM \"{}\"", final_table_name, temporary_table_name);
    match conn.query(&insert_final_query).await {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to insert data into final table: {}", err)),
    }

    // Drop the temporary table
    let drop_temp_table_query = format!("DROP TABLE IF EXISTS \"{}\"", temporary_table_name);
    match conn.query(&drop_temp_table_query).await {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to drop temporary table: {}", err)),
    }

    Ok(line_count)
}
