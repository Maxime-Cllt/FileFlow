pub fn get_drop_statement(db_driver: &str, final_table_name: &str) -> Result<String, String> {
    match &db_driver.to_lowercase()[..] {
        "sqlite" => {
            Ok(format!("DROP TABLE IF EXISTS \"{}\"", final_table_name))
        }
        "mysql" | "mariadb" => {
            Ok(format!("DROP TABLE IF EXISTS `{}`", final_table_name))
        }
        "postgres" => {
            Ok(format!("DROP TABLE IF EXISTS \"{}\"", final_table_name))
        }
        _ => {
            Err("Unsupported database driver".to_string())
        }
    }
}

pub fn get_insert_into_statement(db_driver: &str, final_table_name: &str, columns: &str) -> Result<String, String> {
    return match &db_driver.to_lowercase()[..] {
        "sqlite" => {
            Ok(format!("INSERT INTO \"{}\" ({}) VALUES ", final_table_name, columns))
        }
        "mysql" | "mariadb" => {
            Ok(format!("INSERT INTO `{}` ({}) VALUES ", final_table_name, columns))
        }
        "postgres" => {
            Ok(format!("INSERT INTO \"{}\" ({}) VALUES ", final_table_name, columns))
        }
        _ => {
            Err("Unsupported database driver".to_string())
        }
    };
}

pub fn get_create_statement(driver: &str, final_table_name: &str, snake_case_headers: &Vec<String>) -> Result<String, String> {
    match &driver.to_lowercase()[..] {
        "sqlite" => {
            Ok(format!("CREATE TABLE \"{}\" ({})", final_table_name, snake_case_headers.iter().map(|h| format!("{} TEXT", h)).collect::<Vec<String>>().join(", ")))
        }
        "mysql" | "mariadb" => {
            Ok(format!("CREATE TABLE `{}` ({})", final_table_name, snake_case_headers.iter().map(|h| format!("{} TEXT", h)).collect::<Vec<String>>().join(", ")))
        }
        "postgres" => {
            Ok(format!("CREATE TABLE \"{}\" ({})", final_table_name, snake_case_headers.iter().map(|h| format!("{} TEXT", h)).collect::<Vec<String>>().join(", ")))
        }
        _ => {
            Err("Unsupported database driver".to_string())
        }
    }
}