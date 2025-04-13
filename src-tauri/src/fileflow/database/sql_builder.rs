use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use std::collections::HashMap;

/// This function is used to generate the DROP TABLE statement for different database drivers.
pub fn build_drop_statement_sql(
    db_driver: &DatabaseEngine,
    final_table_name: &str,
) -> Result<String, String> {
    match db_driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
            Ok(format!("DROP TABLE IF EXISTS \"{final_table_name}\""))
        }
        DatabaseEngine::MariaDB | DatabaseEngine::MySQL => {
            Ok(format!("DROP TABLE IF EXISTS `{final_table_name}`"))
        }
    }
}

/// This function is used to generate the INSERT INTO statement for different database drivers.
pub fn build_prepared_statement_sql(
    db_driver: &DatabaseEngine,
    table_name: &str,
    columns: &[String],
) -> String {
    let quote: char = match db_driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => '\"',
        DatabaseEngine::MariaDB | DatabaseEngine::MySQL => '`',
    };
    let mut query = format!("INSERT INTO {quote}{table_name}{quote} (");
    for column in columns.iter() {
        if column.is_empty() {
            continue;
        }
        query.push_str(&format!("{quote}{column}{quote}"));
        if column != columns.last().unwrap() {
            query.push_str(", ");
        }
    }
    query.push_str(") VALUES ");
    query
}

/// This function is used to generate the COPY statement for different database drivers.
pub fn build_copy_table_sql(
    db_driver: &DatabaseEngine,
    temporary_table_name: &str,
    final_table_name: &str,
) -> String {
    let quote: char = match db_driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => '\"',
        DatabaseEngine::MySQL | DatabaseEngine::MariaDB => '`',
    };
    format!(
        "INSERT INTO {quote}{final_table_name}{quote} SELECT * FROM {quote}{temporary_table_name}{quote}"
    )
}

/// This function is used to generate the CREATE TABLE statement with fixed size for each column for different database drivers.
pub fn build_create_with_fixed_size_sql(
    driver: &DatabaseEngine,
    final_table_name: &str,
    map_column_max_length: &HashMap<&str, usize>,
    snake_case_headers: &[String],
) -> String {
    // Constants
    const MAX_VARCHAR_LENGTH: usize = 255;
    const TEXT_TYPE: &str = "TEXT";
    const VARCHAR_TYPE: &str = "VARCHAR";

    // Determine database-specific formatting
    let quote: char = match driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => '"',
        DatabaseEngine::MySQL | DatabaseEngine::MariaDB => '`',
    };

    let mut columns: Vec<String> = Vec::with_capacity(snake_case_headers.len());
    let mut total_length: usize = 0;

    // Build column definitions
    for header in snake_case_headers {
        let max_length = map_column_max_length
            .get(header.as_str())
            .copied()
            .unwrap_or(MAX_VARCHAR_LENGTH);

        // Determine column type
        let type_str: String = if max_length <= MAX_VARCHAR_LENGTH {
            format!("{VARCHAR_TYPE}({max_length})") // VARCHAR(n)
        } else {
            TEXT_TYPE.into()
        };

        // Format column definition
        let column: String = format!("{0}{1}{0} {2}", quote, header, type_str);
        total_length += column.len();
        columns.push(column);
    }

    // Pre-calculate final string capacity
    let table_quoted: String = format!("{0}{1}{0}", quote, final_table_name);
    let mut result: String = String::with_capacity(
        15 + // "CREATE TABLE  ();"
            table_quoted.len() +
            total_length +
            columns.len() * 2, // ", " separators
    );

    // Build the final SQL statement
    result.push_str("CREATE TABLE ");
    result.push_str(&table_quoted);
    result.push_str(" (");
    result.push_str(&columns.join(", "));
    result.push_str(");");

    result
}

/// This function is used to generate the CREATE TABLE statement for different database drivers.
pub fn build_create_table_sql(
    driver: &DatabaseEngine,
    final_table_name: &str,
    snake_case_headers: &[String],
) -> String {
    match driver {
        DatabaseEngine::SQLite | DatabaseEngine::Postgres => format!(
            "CREATE TABLE \"{final_table_name}\" ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("{h} TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        DatabaseEngine::MariaDB | DatabaseEngine::MySQL => format!(
            "CREATE TABLE `{final_table_name}` ({})",
            snake_case_headers
                .iter()
                .map(|h| format!("`{h}` TEXT"))
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}

/// Get the query to fetch all tables from the database for different drivers
pub fn build_query_all_tables(driver: &DatabaseEngine, schema: &str) -> String {
    let query: String = match driver {
        &DatabaseEngine::MySQL | &DatabaseEngine::MariaDB => format!(
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = '{schema}';"
        ),
        &DatabaseEngine::Postgres => {
            "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'public';".into()
        }
        &DatabaseEngine::SQLite => "SELECT name FROM sqlite_master WHERE type='table';".into(),
    };
    query
}

#[cfg(test)]
mod test {
    use crate::fileflow::database::sql_builder::{
        build_create_table_sql, build_create_with_fixed_size_sql, build_drop_statement_sql,
        build_prepared_statement_sql, build_query_all_tables,
    };
    use crate::fileflow::enumeration::database_engine::DatabaseEngine;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_drop_statement() {
        assert_eq!(
            build_drop_statement_sql(&DatabaseEngine::SQLite, "table_name").unwrap(),
            "DROP TABLE IF EXISTS \"table_name\""
        );
        assert_eq!(
            build_drop_statement_sql(&DatabaseEngine::MySQL, "table_name").unwrap(),
            "DROP TABLE IF EXISTS `table_name`"
        );
        assert_eq!(
            build_drop_statement_sql(&DatabaseEngine::Postgres, "table_name").unwrap(),
            "DROP TABLE IF EXISTS \"table_name\""
        );
        assert_eq!(
            build_drop_statement_sql(&DatabaseEngine::SQLite, "").unwrap(),
            "DROP TABLE IF EXISTS \"\""
        );
        assert_eq!(
            build_drop_statement_sql(&DatabaseEngine::MySQL, "").unwrap(),
            "DROP TABLE IF EXISTS ``"
        );
        assert_eq!(
            build_drop_statement_sql(&DatabaseEngine::Postgres, "").unwrap(),
            "DROP TABLE IF EXISTS \"\""
        );
    }

    #[tokio::test]
    async fn test_get_insert_into_statement() {
        assert_eq!(
            build_prepared_statement_sql(
                &DatabaseEngine::SQLite,
                "table_name",
                &["columns".into()]
            ),
            "INSERT INTO \"table_name\" (\"columns\") VALUES "
        );
        assert_eq!(
            build_prepared_statement_sql(&DatabaseEngine::MySQL, "table_name", &["columns".into()]),
            "INSERT INTO `table_name` (`columns`) VALUES "
        );
        assert_eq!(
            build_prepared_statement_sql(
                &DatabaseEngine::Postgres,
                "table_name",
                &["columns".into()]
            ),
            "INSERT INTO \"table_name\" (\"columns\") VALUES "
        );

        assert_eq!(
            build_prepared_statement_sql(&DatabaseEngine::SQLite, "table_name", &["".into()]),
            "INSERT INTO \"table_name\" () VALUES "
        );
        assert_eq!(
            build_prepared_statement_sql(&DatabaseEngine::MySQL, "table_name", &["".into()]),
            "INSERT INTO `table_name` () VALUES "
        );
        assert_eq!(
            build_prepared_statement_sql(&DatabaseEngine::Postgres, "table_name", &["".into()]),
            "INSERT INTO \"table_name\" () VALUES "
        );

        assert_eq!(
            build_prepared_statement_sql(
                &DatabaseEngine::SQLite,
                "table_name",
                &["header1".into(), "header2".into()]
            ),
            "INSERT INTO \"table_name\" (\"header1\", \"header2\") VALUES "
        );
        assert_eq!(
            build_prepared_statement_sql(
                &DatabaseEngine::MySQL,
                "table_name",
                &["header1".into(), "header2".into()]
            ),
            "INSERT INTO `table_name` (`header1`, `header2`) VALUES "
        );
        assert_eq!(
            build_prepared_statement_sql(
                &DatabaseEngine::Postgres,
                "table_name",
                &["header1".into(), "header2".into()]
            ),
            "INSERT INTO \"table_name\" (\"header1\", \"header2\") VALUES "
        );
    }

    #[tokio::test]
    async fn test_get_create_statement() {
        let snake_case_headers: Vec<String> = vec!["header1".into(), "header2".into()];
        assert_eq!(
            build_create_table_sql(&DatabaseEngine::SQLite, "table_name", &snake_case_headers),
            "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)"
        );
        assert_eq!(
            build_create_table_sql(&DatabaseEngine::MySQL, "table_name", &snake_case_headers),
            "CREATE TABLE `table_name` (`header1` TEXT, `header2` TEXT)"
        );
        assert_eq!(
            build_create_table_sql(&DatabaseEngine::Postgres, "table_name", &snake_case_headers),
            "CREATE TABLE \"table_name\" (header1 TEXT, header2 TEXT)"
        );

        let snake_case_headers: Vec<String> = vec!["header1".into()];
        assert_eq!(
            build_create_table_sql(&DatabaseEngine::SQLite, "table_name", &snake_case_headers),
            "CREATE TABLE \"table_name\" (header1 TEXT)"
        );
        assert_eq!(
            build_create_table_sql(&DatabaseEngine::MySQL, "table_name", &snake_case_headers),
            "CREATE TABLE `table_name` (`header1` TEXT)"
        );
        assert_eq!(
            build_create_table_sql(&DatabaseEngine::Postgres, "table_name", &snake_case_headers),
            "CREATE TABLE \"table_name\" (header1 TEXT)"
        );
    }

    #[tokio::test]
    async fn test_get_create_statement_with_fixed_size() {
        const FINAL_TABLE_NAME: &str = "test_table";

        let snake_case_headers: Vec<String> = vec!["header1".into(), "header2".into()];
        let map_max_length: HashMap<&str, usize> =
            snake_case_headers.iter().map(|h| (h.as_str(), 0)).collect();

        let mut db_driver: HashMap<&DatabaseEngine, &str> = HashMap::new();
        db_driver.insert(
            &DatabaseEngine::Postgres,
            "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(0), \"header2\" VARCHAR(0));",
        );
        db_driver.insert(
            &DatabaseEngine::MySQL,
            "CREATE TABLE `test_table` (`header1` VARCHAR(0), `header2` VARCHAR(0));",
        );
        db_driver.insert(
            &DatabaseEngine::MariaDB,
            "CREATE TABLE `test_table` (`header1` VARCHAR(0), `header2` VARCHAR(0));",
        );
        db_driver.insert(
            &DatabaseEngine::SQLite,
            "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(0), \"header2\" VARCHAR(0));",
        );

        let final_columns: Vec<String> = vec!["header1".into(), "header2".into()];

        for (driver, expected) in db_driver {
            let result: String = build_create_with_fixed_size_sql(
                driver,
                FINAL_TABLE_NAME,
                &map_max_length,
                &final_columns,
            );
            assert_eq!(result, expected);
        }

        let map_max_length: HashMap<&str, usize> = snake_case_headers
            .iter()
            .map(|h| (h.as_str(), 10))
            .collect();
        let mut db_driver: HashMap<&DatabaseEngine, &str> = HashMap::new();
        db_driver.insert(
            &DatabaseEngine::Postgres,
            "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(10), \"header2\" VARCHAR(10));",
        );
        db_driver.insert(
            &DatabaseEngine::MySQL,
            "CREATE TABLE `test_table` (`header1` VARCHAR(10), `header2` VARCHAR(10));",
        );
        db_driver.insert(
            &DatabaseEngine::MariaDB,
            "CREATE TABLE `test_table` (`header1` VARCHAR(10), `header2` VARCHAR(10));",
        );
        db_driver.insert(
            &DatabaseEngine::SQLite,
            "CREATE TABLE \"test_table\" (\"header1\" VARCHAR(10), \"header2\" VARCHAR(10));",
        );

        for (driver, expected) in db_driver {
            let result: String = build_create_with_fixed_size_sql(
                driver,
                FINAL_TABLE_NAME,
                &map_max_length,
                &final_columns,
            );
            assert_eq!(result, expected);
        }
    }

    #[tokio::test]
    async fn test_build_query_all_tables() {
        let test_cases: Vec<(&DatabaseEngine, String)> = vec![
            (
                &DatabaseEngine::MySQL,
                "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'test';"
                    .into(),
            ),
            (
                &DatabaseEngine::MariaDB,
                "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'test';"
                    .into(),
            ),
            (
                &DatabaseEngine::Postgres,
                "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'public';"
                    .into(),
            ),
            (
                &DatabaseEngine::SQLite,
                "SELECT name FROM sqlite_master WHERE type='table';".into(),
            ),
        ];

        for (driver, expected) in test_cases {
            assert_eq!(
                build_query_all_tables(driver, "test"),
                expected,
                "Failed for driver: {:?}",
                driver
            );
        }
    }
}
