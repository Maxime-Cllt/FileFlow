use std::fs::File;
use csv::Reader;
use crate::DatabaseConnection;

// Fast insert function (without checking max length)
pub async fn fast_insert(
    conn: &DatabaseConnection,
    reader: &mut Reader<File>,
    snake_case_headers: &Vec<String>,
    final_table_name: &str,
) -> Result<u64, String> {
    let drop_final_table_query = format!("DROP TABLE IF EXISTS \"{}\"", final_table_name);
    match conn.query(&drop_final_table_query).await {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to drop final table: {}", err)),
    }

    let create_table_query = format!(
        "CREATE TABLE \"{}\" ({})",  // Ensure table name is quoted
        final_table_name,
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

    let mut insert_query = format!("INSERT INTO \"{}\" ({}) VALUES", final_table_name, snake_case_headers.join(", "));
    let mut first = true;

    let mut batch_insert = String::new();

    let mut count: u64 = 0;
    let mut line_count: u64 = 0;
    for result in reader.records() {
        let record = result.unwrap();
        let mut values = Vec::new();

        for value in record.iter() {
            let value = value.trim();
            values.push(format!("'{}'", value.replace("'", "''")));  // Escape single quotes
        }

        if first {
            first = false;
        } else {
            batch_insert.push_str(", ");
        }
        batch_insert.push_str(&format!("({})", values.join(", ")));

        count += 1;
        line_count += 1;
        if count == 1000 {
            insert_query.push_str(&batch_insert);
            match conn.query(&insert_query).await {
                Ok(_) => (),
                Err(err) => return Err(format!("Failed to insert data: {}", err)),
            }
            count = 0;
            batch_insert.clear();
        }
    }

    if count > 0 {
        insert_query.push_str(&batch_insert);
        match conn.query(&insert_query).await {
            Ok(_) => (),
            Err(err) => return Err(format!("Failed to insert data: {}", err)),
        }
    }

    Ok(line_count)
}