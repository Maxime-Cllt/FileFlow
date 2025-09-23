use crate::fileflow::enums::database_engine::DatabaseEngine;
use std::collections::HashMap;

/// SQL Builder for generating database-specific SQL statements
pub struct SqlBuilder;

impl SqlBuilder {
    /// Get the appropriate quote character for the database engine
    pub fn get_quote_char(engine: &DatabaseEngine) -> char {
        match engine {
            DatabaseEngine::SQLite | DatabaseEngine::Postgres => '"',
            DatabaseEngine::MariaDB | DatabaseEngine::MySQL => '`',
        }
    }

    /// Generate the DROP TABLE statement
    pub fn build_drop_statement(
        engine: &DatabaseEngine,
        table_name: &str,
    ) -> Result<String, String> {
        if table_name.trim().is_empty() {
            return Err("Table name cannot be empty".to_string());
        }

        let quote = Self::get_quote_char(engine);
        Ok(format!("DROP TABLE IF EXISTS {quote}{table_name}{quote}"))
    }

    /// Generate the INSERT INTO statement
    pub fn build_prepared_statement(
        engine: &DatabaseEngine,
        table_name: &str,
        columns: &[String],
    ) -> Result<String, String> {
        if table_name.trim().is_empty() {
            return Err("Table name cannot be empty".to_string());
        }

        let quote = Self::get_quote_char(engine);

        // Filter out empty columns to fix the bug
        let valid_columns: Vec<&String> = columns
            .iter()
            .filter(|col| !col.trim().is_empty())
            .collect();

        if valid_columns.is_empty() {
            return Err("At least one valid column name is required".to_string());
        }

        let mut query = format!("INSERT INTO {quote}{table_name}{quote} (");

        // Join columns with proper comma separation
        let column_list = valid_columns
            .iter()
            .map(|col| format!("{quote}{}{quote}", col.trim()))
            .collect::<Vec<String>>()
            .join(", ");

        query.push_str(&column_list);
        query.push_str(") VALUES ");

        Ok(query)
    }

    /// Generate the COPY statement
    pub fn build_copy_table(
        engine: &DatabaseEngine,
        temporary_table_name: &str,
        final_table_name: &str,
    ) -> Result<String, String> {
        if temporary_table_name.trim().is_empty() || final_table_name.trim().is_empty() {
            return Err("Table names cannot be empty".to_string());
        }

        let quote = Self::get_quote_char(engine);
        Ok(format!(
            "INSERT INTO {quote}{final_table_name}{quote} SELECT * FROM {quote}{temporary_table_name}{quote}"
        ))
    }

    /// Generate the CREATE TABLE statement with fixed size for each column
    pub fn build_create_with_fixed_size(
        engine: &DatabaseEngine,
        table_name: &str,
        column_max_lengths: &HashMap<&str, usize>,
        headers: &[String],
    ) -> Result<String, String> {
        if table_name.trim().is_empty() {
            return Err("Table name cannot be empty".to_string());
        }

        if headers.is_empty() {
            return Err("At least one column header is required".to_string());
        }

        const MAX_VARCHAR_LENGTH: usize = 255;
        const TEXT_TYPE: &str = "TEXT";
        const VARCHAR_TYPE: &str = "VARCHAR";

        let quote = Self::get_quote_char(engine);
        let mut columns = Vec::with_capacity(headers.len());

        // Build column definitions
        for header in headers.iter() {
            let header = header.trim();
            if header.is_empty() {
                continue; // Skip empty headers
            }

            let max_length = column_max_lengths
                .get(header)
                .copied()
                .unwrap_or(MAX_VARCHAR_LENGTH);

            // Determine column type
            let type_str = if max_length <= MAX_VARCHAR_LENGTH && max_length > 0 {
                format!("{VARCHAR_TYPE}({max_length})")
            } else {
                TEXT_TYPE.to_string()
            };

            // Format column definition
            columns.push(format!("{quote}{header}{quote} {type_str}"));
        }

        if columns.is_empty() {
            return Err("No valid column headers provided".to_string());
        }

        // Build the final SQL statement
        Ok(format!(
            "CREATE TABLE {quote}{table_name}{quote} ({});",
            columns.join(", ")
        ))
    }

    /// Generate the CREATE TABLE statement with TEXT type for all columns
    pub fn build_create_table(
        engine: &DatabaseEngine,
        table_name: &str,
        headers: &[String],
    ) -> Result<String, String> {
        if table_name.trim().is_empty() {
            return Err("Table name cannot be empty".to_string());
        }

        if headers.is_empty() {
            return Err("At least one column header is required".to_string());
        }

        let quote = Self::get_quote_char(engine);

        // Filter out empty headers and build column definitions
        let valid_headers: Vec<&String> = headers.iter().filter(|h| !h.trim().is_empty()).collect();

        if valid_headers.is_empty() {
            return Err("No valid column headers provided".to_string());
        }

        let columns = valid_headers
            .iter()
            .map(|h| format!("{quote}{}{quote} TEXT", h.trim()))
            .collect::<Vec<String>>()
            .join(", ");

        Ok(format!(
            "CREATE TABLE {quote}{table_name}{quote} ({columns})"
        ))
    }

    /// Get the query to fetch all tables from the database
    pub fn build_query_all_tables(engine: &DatabaseEngine, schema: &str) -> String {
        match engine {
            DatabaseEngine::MySQL | DatabaseEngine::MariaDB => {
                format!("SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = '{schema}';")
            }
            DatabaseEngine::Postgres => {
                "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = 'public';"
                    .to_string()
            }
            DatabaseEngine::SQLite => {
                "SELECT name FROM sqlite_master WHERE type='table';".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// List of database engines to test
    const ENGINES: [DatabaseEngine; 4] = [
        DatabaseEngine::SQLite,
        DatabaseEngine::Postgres,
        DatabaseEngine::MySQL,
        DatabaseEngine::MariaDB,
    ];

    #[tokio::test]
    async fn test_sql_builder_drop_statement() {
        for engine in ENGINES {
            let table_name = "test_table";
            let expected = match engine {
                DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
                    format!("DROP TABLE IF EXISTS \"{table_name}\"")
                }
                DatabaseEngine::MySQL | DatabaseEngine::MariaDB => {
                    format!("DROP TABLE IF EXISTS `{table_name}`")
                }
            };
            assert_eq!(
                SqlBuilder::build_drop_statement(&engine, table_name).unwrap(),
                expected
            );
        }

        // Test error case
        assert!(SqlBuilder::build_drop_statement(&DatabaseEngine::SQLite, "").is_err());
        assert!(SqlBuilder::build_drop_statement(&DatabaseEngine::MySQL, "   ").is_err());
    }

    #[tokio::test]
    async fn test_quote_characters() {
        for engine in ENGINES {
            let quote = SqlBuilder::get_quote_char(&engine);
            let expected = match engine {
                DatabaseEngine::SQLite | DatabaseEngine::Postgres => '"',
                DatabaseEngine::MySQL | DatabaseEngine::MariaDB => '`',
            };
            assert_eq!(quote, expected);
        }
    }

    #[tokio::test]
    async fn test_sql_builder_prepared_statement() {
        for engine in ENGINES {
            let table_name = "test_table";
            let columns = vec!["col1".to_string(), "col2".to_string(), "col3".to_string()];
            let expected = match engine {
                DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
                    format!("INSERT INTO \"{table_name}\" (\"col1\", \"col2\", \"col3\") VALUES ")
                }
                DatabaseEngine::MySQL | DatabaseEngine::MariaDB => {
                    format!("INSERT INTO `{table_name}` (`col1`, `col2`, `col3`) VALUES ")
                }
            };
            assert_eq!(
                SqlBuilder::build_prepared_statement(&engine, table_name, &columns).unwrap(),
                expected
            );
        }

        // Test error cases
        assert!(SqlBuilder::build_prepared_statement(
            &DatabaseEngine::SQLite,
            "",
            &["col1".to_string()]
        )
        .is_err());
        assert!(SqlBuilder::build_prepared_statement(
            &DatabaseEngine::MySQL,
            "   ",
            &["col1".to_string()]
        )
        .is_err());
        assert!(
            SqlBuilder::build_prepared_statement(&DatabaseEngine::Postgres, "test_table", &[])
                .is_err()
        );
        assert!(SqlBuilder::build_prepared_statement(
            &DatabaseEngine::MariaDB,
            "test_table",
            &["   ".to_string()]
        )
        .is_err());
    }

    #[tokio::test]
    async fn test_sql_builder_copy_table() {
        for engine in ENGINES {
            let temp_table = "temp_table";
            let final_table = "final_table";
            let expected = match engine {
                DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
                    format!("INSERT INTO \"{final_table}\" SELECT * FROM \"{temp_table}\"")
                }
                DatabaseEngine::MySQL | DatabaseEngine::MariaDB => {
                    format!("INSERT INTO `{final_table}` SELECT * FROM `{temp_table}`")
                }
            };
            assert_eq!(
                SqlBuilder::build_copy_table(&engine, temp_table, final_table).unwrap(),
                expected
            );
        }

        // Test error cases
        assert!(SqlBuilder::build_copy_table(&DatabaseEngine::SQLite, "", "final_table").is_err());
        assert!(SqlBuilder::build_copy_table(&DatabaseEngine::MySQL, "temp_table", "").is_err());
        assert!(SqlBuilder::build_copy_table(&DatabaseEngine::Postgres, "   ", "   ").is_err());
    }

    #[tokio::test]
    async fn test_sql_builder_create_with_fixed_size() {
        use std::collections::HashMap;

        for engine in ENGINES {
            let table_name = "test_table";
            let headers = vec![
                "col1".to_string(),
                "col2".to_string(),
                "col3".to_string(),
                "col4".to_string(),
            ];
            let mut column_max_lengths = HashMap::new();
            column_max_lengths.insert("col1", 100);
            column_max_lengths.insert("col2", 300); // Should default to TEXT
            column_max_lengths.insert("col3", 255);
            // col4 not specified, should default to VARCHAR(255)

            let expected = match engine {
                DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
                    format!(
                        "CREATE TABLE \"{table_name}\" (\"col1\" VARCHAR(100), \"col2\" TEXT, \"col3\" VARCHAR(255), \"col4\" VARCHAR(255));"
                    )
                }
                DatabaseEngine::MySQL | DatabaseEngine::MariaDB => {
                    format!(
                        "CREATE TABLE `{table_name}` (`col1` VARCHAR(100), `col2` TEXT, `col3` VARCHAR(255), `col4` VARCHAR(255));"
                    )
                }
            };
            assert_eq!(
                SqlBuilder::build_create_with_fixed_size(
                    &engine,
                    table_name,
                    &column_max_lengths,
                    &headers
                )
                .unwrap(),
                expected
            );
        }

        // Test error cases
        let empty_map: HashMap<&str, usize> = HashMap::new();
        assert!(SqlBuilder::build_create_with_fixed_size(
            &DatabaseEngine::SQLite,
            "",
            &empty_map,
            &["col1".to_string()]
        )
        .is_err());
        assert!(SqlBuilder::build_create_with_fixed_size(
            &DatabaseEngine::MySQL,
            "   ",
            &empty_map,
            &["col1".to_string()]
        )
        .is_err());
        assert!(SqlBuilder::build_create_with_fixed_size(
            &DatabaseEngine::Postgres,
            "test_table",
            &empty_map,
            &[]
        )
        .is_err());
        assert!(SqlBuilder::build_create_with_fixed_size(
            &DatabaseEngine::MariaDB,
            "test_table",
            &empty_map,
            &["   ".to_string()]
        )
        .is_err());
    }

    #[tokio::test]
    async fn test_sql_builder_create_table() {
        for engine in ENGINES {
            let table_name = "test_table";
            let headers = vec!["col1".to_string(), "col2".to_string(), "col3".to_string()];
            let expected = match engine {
                DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
                    format!(
                        "CREATE TABLE \"{table_name}\" (\"col1\" TEXT, \"col2\" TEXT, \"col3\" TEXT)"
                    )
                }
                DatabaseEngine::MySQL | DatabaseEngine::MariaDB => {
                    format!("CREATE TABLE `{table_name}` (`col1` TEXT, `col2` TEXT, `col3` TEXT)")
                }
            };
            assert_eq!(
                SqlBuilder::build_create_table(&engine, table_name, &headers).unwrap(),
                expected
            );
        }
    }

    #[tokio::test]
    async fn test_sql_builder_create_table_with_empty_headers() {
        for engine in ENGINES {
            let table_name = "test_table";
            let headers = vec!["col1".to_string(), "   ".to_string(), "".to_string(), "col3".to_string()];
            let expected = match engine {
                DatabaseEngine::SQLite | DatabaseEngine::Postgres => {
                    format!(
                        "CREATE TABLE \"{table_name}\" (\"col1\" TEXT, \"col3\" TEXT)"
                    )
                }
                DatabaseEngine::MySQL | DatabaseEngine::MariaDB => {
                    format!("CREATE TABLE `{table_name}` (`col1` TEXT, `col3` TEXT)")
                }
            };
            assert_eq!(
                SqlBuilder::build_create_table(&engine, table_name, &headers).unwrap(),
                expected
            );
        }

        // Test error case with all empty headers
        for engine in ENGINES {
            let table_name = "test_table";
            let headers = vec!["   ".to_string(), "".to_string()];
            assert!(SqlBuilder::build_create_table(&engine, table_name, &headers).is_err());
        }
    }
}
