use std::collections::HashMap;
use crate::fileflow::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};


/**
 * This function is used to generate the DROP TABLE statement for different database drivers.
 */
pub fn get_drop_statement(db_driver: &str, final_table_name: &str) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        SQLITE => {
            Ok(format!("DROP TABLE IF EXISTS \"{}\"", final_table_name))
        }
        MYSQL | MARIADB => {
            Ok(format!("DROP TABLE IF EXISTS `{}`", final_table_name))
        }
        POSTGRES => {
            Ok(format!("DROP TABLE IF EXISTS \"{}\"", final_table_name))
        }
        _ => {
            Err("Unsupported database driver".to_string())
        }
    }
}

/**
 * This function is used to generate the INSERT INTO statement for different database drivers.
 */
pub fn get_insert_into_statement(db_driver: &str, final_table_name: &str, columns: &str) -> Result<String, String> {
    return match &db_driver.to_lowercase()[..] {
        SQLITE => {
            Ok(format!("INSERT INTO \"{}\" ({}) VALUES ", final_table_name, columns))
        }
        MYSQL | MARIADB => {
            Ok(format!("INSERT INTO `{}` ({}) VALUES ", final_table_name, columns))
        }
        POSTGRES => {
            Ok(format!("INSERT INTO \"{}\" ({}) VALUES ", final_table_name, columns))
        }
        _ => {
            Err("Unsupported database driver".to_string())
        }
    };
}

pub fn get_create_statement(driver: &str, final_table_name: &str, snake_case_headers: &Vec<String>) -> Result<String, String> {
    match &driver.to_lowercase()[..] {
        SQLITE => {
            Ok(format!("CREATE TABLE \"{}\" ({})", final_table_name, snake_case_headers.iter().map(|h| format!("{} TEXT", h)).collect::<Vec<String>>().join(", ")))
        }
        MYSQL | MARIADB => {
            Ok(format!("CREATE TABLE `{}` ({})", final_table_name, snake_case_headers.iter().map(|h| format!("{} TEXT", h)).collect::<Vec<String>>().join(", ")))
        }
        POSTGRES => {
            Ok(format!("CREATE TABLE \"{}\" ({})", final_table_name, snake_case_headers.iter().map(|h| format!("{} TEXT", h)).collect::<Vec<String>>().join(", ")))
        }
        _ => {
            Err("Unsupported database driver".to_string())
        }
    }
}

/**
 * This function is used to generate the CREATE TABLE statement with fixed size for each column for different database drivers.
 */
pub fn get_create_statement_with_fixed_size(driver: &str, final_table_name: &str, map_max_length: &HashMap<&str, usize>, snake_case_headers: &Vec<String>) -> Result<String, String> {
    match driver {
        POSTGRES => {
            Ok(format!(
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
            ))
        }
        MYSQL | MARIADB => {
            Ok(format!(
                "CREATE TABLE `{}` ({})",
                final_table_name,
                snake_case_headers
                    .iter()
                    .map(|header| {
                        let max_length = map_max_length.get(header.as_str()).unwrap();
                        format!("`{}` VARCHAR({})", header, max_length)
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
        }
        SQLITE => {
            Ok(format!(
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
            ))
        }
        _ => {
            Err(format!("Unsupported database driver: {}", driver))
        }
    }
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
    headers.iter().map(|h| h.to_lowercase().replace(" ", "_")).collect()
}