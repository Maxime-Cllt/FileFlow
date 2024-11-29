use crate::fileflow::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use std::collections::HashMap;

/**
 * This function is used to generate the DROP TABLE statement for different database drivers.
 */
pub fn get_drop_statement(db_driver: &str, final_table_name: &str) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!("DROP TABLE IF EXISTS \"{}\"", final_table_name)),
        MYSQL | MARIADB => Ok(format!("DROP TABLE IF EXISTS `{}`", final_table_name)),
        _ => Err("Unsupported database driver".to_string()),
    }
}

/**
 * This function is used to generate the INSERT INTO statement for different database drivers.
 */
pub fn get_insert_into_statement(
    db_driver: &str,
    final_table_name: &str,
    columns: &str,
) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!(
            "INSERT INTO \"{}\" ({}) VALUES ",
            final_table_name, columns
        )),
        MYSQL | MARIADB => Ok(format!(
            "INSERT INTO `{}` ({}) VALUES ",
            final_table_name, columns
        )),
        _ => Err("Unsupported database driver".to_string()),
    }
}

/**
 * This function is used to generate the INSERT INTO statement with fixed size for each column for different database drivers.
 */
pub fn get_create_statement(
    driver: &str,
    final_table_name: &str,
    snake_case_headers: &Vec<String>,
) -> Result<String, String> {
    match &driver.to_lowercase()[..] {
        SQLITE | POSTGRES => Ok(format!(
            "CREATE TABLE \"{}\" ({})",
            final_table_name,
            snake_case_headers
                .iter()
                .map(|h| format!("{} TEXT", h))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        MYSQL | MARIADB => Ok(format!(
            "CREATE TABLE `{}` ({})",
            final_table_name,
            snake_case_headers
                .iter()
                .map(|h| format!("{} TEXT", h))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        _ => Err("Unsupported database driver".to_string()),
    }
}

/**
 * This function is used to generate the CREATE TABLE statement with fixed size for each column for different database drivers.
 */
pub fn get_create_statement_with_fixed_size(
    driver: &str,
    final_table_name: &str,
    map_column_max_length: &HashMap<&str, usize>,
    snake_case_headers: &Vec<String>,
) -> Result<String, String> {
    // Start building the SQL statement based on the driver
    let mut create_table_sql = match driver {
        SQLITE|POSTGRES => format!("CREATE TABLE \"{}\" (", final_table_name),
        MYSQL | MARIADB => format!("CREATE TABLE `{}` (", final_table_name),
        _ => return Err("Unsupported database driver".to_string()),
    };

    // Iterate over snake_case_headers to maintain the order of columns
    for header in snake_case_headers.iter() {
        // Get the max length from map_column_max_length and determine column type
        let max_length = match map_column_max_length.get(header.as_str()) {
            Some(&length) => length,
            None => return Err(format!("Column {} not found in max length map", header)),
        };

        let column_type = if max_length < 256 {
            format!("VARCHAR({})", max_length)
        } else {
            "TEXT".to_string()
        };

        // Quote column names as required by each driver
        let quoted_column = match driver {
            SQLITE | POSTGRES => format!("\"{}\"", header),
            MYSQL | MARIADB => format!("`{}`", header),
            _ => return Err("Unsupported database driver".to_string()),
        };

        // Append the column definition to the SQL statement
        create_table_sql.push_str(&format!("{} {}, ", quoted_column, column_type));
    }

    // Remove the trailing comma and space, then close the SQL statement
    if create_table_sql.ends_with(", ") {
        create_table_sql.pop();
        create_table_sql.pop();
    }
    create_table_sql.push_str(");");

    Ok(create_table_sql)
}

/**
 * This function is used to format the column names to snake_case and replace empty column names with column_1, column_2, etc.
*/
pub fn get_formated_column_names(headers: Vec<String>) -> Vec<String> {
    let mut headers: Vec<String> = headers;
    for i in 0..headers.len() {
        if headers[i].trim().len() == 0 {
            headers[i] = format!("column_{}", i + 1);
        }
    }
    headers
        .iter()
        .map(|h| h.to_lowercase().replace(" ", "_"))
        .collect()
}
