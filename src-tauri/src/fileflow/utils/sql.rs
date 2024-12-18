use crate::fileflow::database::connection::Connection;
use crate::fileflow::utils::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use csv::StringRecord;
use std::collections::HashMap;

/// This function is used to generate the DROP TABLE statement for different database drivers.
pub fn get_drop_statement(db_driver: &str, final_table_name: &str) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!("DROP TABLE IF EXISTS \"{final_table_name}\"")),
        MYSQL | MARIADB => Ok(format!("DROP TABLE IF EXISTS `{final_table_name}`")),
        _ => Err("Unsupported database driver".to_string()),
    }
}

/// This function is used to generate the INSERT INTO statement for different database drivers.
pub fn get_insert_into_statement(
    db_driver: &str,
    final_table_name: &str,
    columns: &str,
) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!(
            "INSERT INTO \"{final_table_name}\" ({columns}) VALUES "
        )),
        MYSQL | MARIADB => Ok(format!(
            "INSERT INTO `{final_table_name}` ({columns}) VALUES "
        )),
        _ => Err("Unsupported database driver".to_string()),
    }
}

/// This function is used to generate the COPY statement for different database drivers.
pub fn get_copy_temp_to_final_table(
    db_driver: &str,
    temporary_table_name: &str,
    final_table_name: &str,
) -> Result<String, String> {
    match db_driver {
        SQLITE | POSTGRES => Ok(format!(
            "INSERT INTO \"{final_table_name}\" SELECT * FROM \"{temporary_table_name}\""
        )),
        MYSQL | MARIADB => Ok(format!(
            "INSERT INTO `{final_table_name}` SELECT * FROM `{temporary_table_name}`"
        )),
        _ => Err(format!("Unsupported database driver: {db_driver}")),
    }
}

/// This function is used to generate the CREATE TABLE statement with fixed size for each column for different database drivers.
pub fn get_create_statement_with_fixed_size(
    driver: &str,
    final_table_name: &str,
    map_column_max_length: &HashMap<&str, usize>,
    snake_case_headers: &[String],
) -> Result<String, String> {
    const MAX_VARCHAR_LENGTH: usize = 255;
    const TEXT_TYPE: &str = "TEXT";
    const VARCHAR_TYPE: &str = "VARCHAR";

    // Start building the SQL statement based on the driver
    let mut create_table_sql: String = match driver {
        SQLITE | POSTGRES => format!("CREATE TABLE \"{final_table_name}\" ("),
        MYSQL | MARIADB => format!("CREATE TABLE `{final_table_name}` ("),
        _ => return Err("Unsupported database driver".to_string()),
    };

    for header in snake_case_headers {
        let max_length: usize = if let Some(&length) = map_column_max_length.get(header.as_str()) {
            length
        } else {
            MAX_VARCHAR_LENGTH
        };

        let column_type: String = if max_length <= MAX_VARCHAR_LENGTH {
            format!("{VARCHAR_TYPE}({max_length})")
        } else {
            TEXT_TYPE.to_string()
        };

        // Quote column names as required by each driver
        let quoted_column: String = match driver {
            SQLITE | POSTGRES => format!("\"{header}\""),
            MYSQL | MARIADB => format!("`{header}`"),
            _ => return Err("Unsupported database driver".to_string()),
        };

        // Append the column definition to the SQL statement
        create_table_sql.push_str(&format!("{quoted_column} {column_type}, "));
    }

    // Remove the trailing comma and space, then close the SQL statement
    if create_table_sql.ends_with(", ") {
        create_table_sql.pop();
        create_table_sql.pop();
    }
    create_table_sql.push_str(");");

    Ok(create_table_sql)
}

/// This function is used to generate the CREATE TABLE statement for different database drivers.
pub fn get_create_statement(
    driver: &str,
    final_table_name: &str,
    snake_case_headers: &[String],
) -> Result<String, String> {
    match &driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!(
            "CREATE TABLE \"{final_table_name}\" ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("{h} TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        MYSQL | MARIADB => Ok(format!(
            "CREATE TABLE `{final_table_name}` ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("{h} TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        _ => Err("Unsupported database driver".to_string()),
    }
}

/// Escape values for SQL insert statement to avoid SQL injection attacks and other issues with special characters in values.
pub fn escaped_values(values: StringRecord) -> String {
    let vec: Vec<String> = values
        .iter()
        .map(|v| format!("'{}'", v.trim().replace("'", "''")))
        .map(|v| v.replace("\\", "\\\\"))
        .collect();
    vec.join(", ")
}

/// Helper function to drop a table if it exists
pub async fn drop_table_if_exists(
    connection: &Connection,
    db_driver: &str,
    table_name: &str,
) -> Result<(), String> {
    let drop_query: &str = &get_drop_statement(db_driver, table_name)?;
    execute_query(
        connection,
        drop_query,
        &format!("Failed to drop table '{table_name}'"),
    )
    .await
}

/// Helper function to execute a query and handle errors
pub async fn execute_query(
    connection: &Connection,
    query: &str,
    context: &str,
) -> Result<(), String> {
    connection
        .query(query)
        .await
        .map_err(|err| format!("{context}: {err}"))
}

/// Helper function to batch-insert records into a table
pub async fn batch_insert(
    connection: &Connection,
    insert_query_base: &str,
    batch: &[String],
    context: &str,
) -> Result<(), String> {
    if batch.is_empty() {
        return Ok(());
    }
    let insert_query: String = format!("{insert_query_base}{}", batch.join(", "));
    execute_query(connection, &insert_query, context).await
}
