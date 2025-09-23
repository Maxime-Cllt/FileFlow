use crate::fileflow::enums::database_engine::DatabaseEngine;
use crate::fileflow::stuct::database::database_connection::DatabaseConnection;
use crate::fileflow::stuct::database::sql_builder::SqlBuilder;
use std::collections::HashMap;

/// Database operations utility struct
pub struct DatabaseAction;

impl DatabaseAction {
    /// Helper function to drop a table if it exists
    pub async fn drop_table_if_exists(
        connection: &DatabaseConnection,
        db_driver: &DatabaseEngine,
        table_name: &str,
    ) -> Result<(), String> {
        Self::validate_table_name(table_name).map_err(|e| format!("Invalid table name: {}", e))?;
        let drop_query: String = SqlBuilder::build_drop_statement(db_driver, table_name)?;
        Self::execute_query(
            connection,
            &drop_query,
            &format!("Failed to drop table '{table_name}'"),
        )
        .await
    }

    /// Helper function to execute a query and handle errors
    pub async fn execute_query(
        connection: &DatabaseConnection,
        query: &str,
        context: &str,
    ) -> Result<(), String> {
        if query.trim().is_empty() {
            return Err(format!("{context}: Query cannot be empty"));
        }

        connection
            .query(query)
            .await
            .map_err(|err| format!("{context}: {err}"))
    }

    /// Helper function to batch-insert records into a table
    pub async fn batch_insert(
        connection: &DatabaseConnection,
        insert_query_base: &str,
        batch: &[String],
        context: &str,
    ) -> Result<(), String> {
        if batch.is_empty() {
            return Ok(());
        }

        if insert_query_base.trim().is_empty() {
            return Err(format!("{context}: Base query cannot be empty"));
        }

        let values_clause = batch.join(", ");
        let insert_query = format!("{insert_query_base}{values_clause}");

        Self::execute_query(connection, &insert_query, context).await
    }

    /// Create the final table and copy data from the temporary table
    pub async fn create_and_copy_final_table(
        connection: &DatabaseConnection,
        db_driver: &DatabaseEngine,
        final_table_name: &str,
        temporary_table_name: &str,
        columns_size_map: &HashMap<&str, usize>,
        final_columns_name: &[String],
    ) -> Result<(), String> {
        Self::validate_table_name(final_table_name)
            .map_err(|e| format!("Invalid final table name: {}", e))?;
        Self::validate_table_name(temporary_table_name)
            .map_err(|e| format!("Invalid temporary table name: {}", e))?;

        if final_columns_name.is_empty() {
            return Err("Column names cannot be empty".to_string());
        }

        // Create the final table
        let create_final_table_query = SqlBuilder::build_create_with_fixed_size(
            db_driver,
            final_table_name,
            columns_size_map,
            final_columns_name,
        )?;

        Self::execute_query(
            connection,
            &create_final_table_query,
            "Failed to create final table",
        )
        .await?;

        // Copy data from temporary to final table
        let copy_data_query =
            SqlBuilder::build_copy_table(db_driver, temporary_table_name, final_table_name)?;

        Self::execute_query(
            connection,
            &copy_data_query,
            "Failed to copy data to final table",
        )
        .await?;

        Ok(())
    }

    /// Drop a list of tables if they exist
    pub async fn drop_existing_tables(
        connection: &DatabaseConnection,
        table_names: &[&str],
        db_driver: &DatabaseEngine,
    ) -> Result<(), String> {
        if table_names.is_empty() {
            return Ok(());
        }

        let mut errors = Vec::new();

        for table_name in table_names {
            if let Err(e) = Self::drop_table_if_exists(connection, db_driver, table_name).await {
                eprintln!("Warning: Failed to drop table '{}': {}", table_name, e);
                errors.push(format!("Table '{}': {}", table_name, e));
            }
        }

        if !errors.is_empty() {
            return Err(format!("Failed to drop some tables: {}", errors.join("; ")));
        }

        Ok(())
    }

    // Private helper methods

    /// Validates that the table name is not empty or just whitespace
    fn validate_table_name(table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if table_name.trim().is_empty() {
            return Err("Table name cannot be empty".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_table_name() {
        assert!(DatabaseAction::validate_table_name("valid_table").is_ok());
        assert!(DatabaseAction::validate_table_name("").is_err());
        assert!(DatabaseAction::validate_table_name("   ").is_err());
    }
}
