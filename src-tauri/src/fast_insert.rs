use std::fs::File;
use csv::Reader;
use crate::DatabaseConnection;

pub async fn fast_insert(
    conn: &DatabaseConnection,
    reader: &mut Reader<File>,
    snake_case_headers: &Vec<String>,
    final_table_name: &str,
) -> Result<u64, String> {

    let drop_final_table_query = format!("DROP TABLE IF EXISTS \"{}\"", final_table_name);
    if let Err(err) = conn.query(&drop_final_table_query).await {
        return Err(format!("Failed to drop final table: {}", err));
    }

    let create_table_query = format!(
        "CREATE TABLE \"{}\" ({})",
        final_table_name,
        snake_case_headers
            .iter()
            .map(|h| format!("{} TEXT", h))
            .collect::<Vec<String>>()
            .join(", ")
    );
    if let Err(err) = conn.query(&create_table_query).await {
        return Err(format!("Failed to create table: {}", err));
    }

    let columns = snake_case_headers.join(", ");
    let mut line_count: u64 = 0;
    let max_batch_size = 5000;
    let mut batch = Vec::with_capacity(max_batch_size);

    let insert_query_base = format!("INSERT INTO \"{}\" ({}) VALUES ", final_table_name, columns);

    for result in reader.records() {
        let record = result.unwrap();

        let values: Vec<String> = record.iter()
            .map(|v| format!("'{}'", v.trim().replace("'", "''")))
            .collect();

        batch.push(format!("({})", values.join(", ")));

        if batch.len() == max_batch_size {
            let insert_query = format!("{}{}", insert_query_base, batch.join(", "));
            if let Err(err) = conn.query(&insert_query).await {
                return Err(format!("Failed to insert data: {}", err));
            }
            line_count += batch.len() as u64;
            batch.clear();
        }
    }

    if !batch.is_empty() {
        let insert_query = format!("{}{}", insert_query_base, batch.join(", "));
        if let Err(err) = conn.query(&insert_query).await {
            return Err(format!("Failed to insert data: {}", err));
        }
        line_count += batch.len() as u64;
    }

    Ok(line_count)
}
