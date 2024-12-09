use crate::fileflow::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

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
    snake_case_headers: &[String],
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
    snake_case_headers: &[String],
) -> Result<String, String> {
    // Start building the SQL statement based on the driver
    let mut create_table_sql = match driver {
        SQLITE | POSTGRES => format!("CREATE TABLE \"{}\" (", final_table_name),
        MYSQL | MARIADB => format!("CREATE TABLE `{}` (", final_table_name),
        _ => return Err("Unsupported database driver".to_string()),
    };

    const MAX_VARCHAR_LENGTH: usize = 255;
    const TEXT_TYPE: &str = "TEXT";
    const VARCHAR_TYPE: &str = "VARCHAR";

    for header in snake_case_headers.iter() {
        let max_length = match map_column_max_length.get(header.as_str()) {
            Some(&length) => length,
            None => return Err(format!("Column {} not found in max length map", header)),
        };

        let column_type = if max_length <= MAX_VARCHAR_LENGTH {
            format!("{}({})", VARCHAR_TYPE, max_length)
        } else {
            TEXT_TYPE.to_string()
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
    const COLUMN_PREFIX: &str = "column_";
    for (i, item) in headers.iter_mut().enumerate() {
        if item.trim().is_empty() {
            *item = format!("{}{}",COLUMN_PREFIX, i + 1);
        }
    }
    headers
        .iter()
        .map(|h| h.to_lowercase().replace(" ", "_"))
        .collect()
}

/// This function is used to detect the separator in a CSV file.
pub fn detect_separator_in_file(file_path: &str) -> io::Result<char> {
    let file: File = File::open(file_path)?;

    if !file_path.ends_with(".csv") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File is not a CSV file",
        ));
    }

    const POSSIBLE_SEPARATORS: [char; 5] = [',', ';', '\t', '|', ' '];

    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer)?;

    for sep in &POSSIBLE_SEPARATORS {
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(*sep as u8)
            .has_headers(false)
            .from_reader(buffer.as_bytes());

        if csv_reader.records().next().is_some() {
            return Ok(*sep);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Could not detect a valid separator",
    ))
}
