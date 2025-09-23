use crate::fileflow::enums::query_rows::QueryRows;
use sqlx::{Column, Row};

/// Extracts data from different database row types
pub(crate) struct RowExtractor;

impl RowExtractor {
    /// Extracts column names and row data from the given QueryRows enum
    pub(crate) fn extract_data(
        query_result: QueryRows,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn std::error::Error>> {
        match query_result {
            QueryRows::MySQL(rows) => Self::extract_mysql_data(rows),
            QueryRows::Postgres(rows) => Self::extract_postgres_data(rows),
            QueryRows::SQLite(rows) => Self::extract_sqlite_data(rows),
        }
    }

    /// Extracts column names and row data from MySQL rows
    fn extract_mysql_data(
        rows: Vec<sqlx::mysql::MySqlRow>,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn std::error::Error>> {
        if rows.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }

        let columns: Vec<String> = Self::extract_column_names(&rows[0]); // Extract column names
        let data: Vec<Vec<String>> = Self::extract_mysql_row_data(rows)?; // Extract row data

        Ok((columns, data))
    }

    /// Extracts column names and row data from PostgreSQL rows
    fn extract_postgres_data(
        rows: Vec<sqlx::postgres::PgRow>,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn std::error::Error>> {
        if rows.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }

        let columns = Self::extract_column_names(&rows[0]);
        let data = Self::extract_postgres_row_data(rows)?;

        Ok((columns, data))
    }

    /// Extracts column names and row data from SQLite rows
    fn extract_sqlite_data(
        rows: Vec<sqlx::sqlite::SqliteRow>,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn std::error::Error>> {
        if rows.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }

        let columns = Self::extract_column_names(&rows[0]);
        let data = Self::extract_sqlite_row_data(rows)?;

        Ok((columns, data))
    }

    /// Extracts column names from a row
    fn extract_column_names<R: Row>(row: &R) -> Vec<String> {
        row.columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect()
    }

    /// Extracts row data from MySQL rows
    fn extract_mysql_row_data(
        rows: Vec<sqlx::mysql::MySqlRow>,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut result = Vec::with_capacity(rows.len());

        for row in rows {
            let mut row_data = Vec::with_capacity(row.len());

            for (idx, column) in row.columns().iter().enumerate() {
                let value = Self::get_mysql_column_value(&row, idx, column.name())?;
                row_data.push(value);
            }

            result.push(row_data);
        }

        Ok(result)
    }

    /// Extracts row data from PostgreSQL rows
    fn extract_postgres_row_data(
        rows: Vec<sqlx::postgres::PgRow>,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut result = Vec::with_capacity(rows.len());

        for row in rows {
            let mut row_data = Vec::with_capacity(row.len());

            for (idx, _) in row.columns().iter().enumerate() {
                let value = Self::get_postgres_column_value(&row, idx)?;
                row_data.push(value);
            }

            result.push(row_data);
        }

        Ok(result)
    }

    /// Extracts row data from SQLite rows
    fn extract_sqlite_row_data(
        rows: Vec<sqlx::sqlite::SqliteRow>,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut result = Vec::with_capacity(rows.len());

        for row in rows {
            let mut row_data = Vec::with_capacity(row.len());

            for (idx, column) in row.columns().iter().enumerate() {
                let value = Self::get_sqlite_column_value(&row, idx, column.name())?;
                row_data.push(value);
            }

            result.push(row_data);
        }

        Ok(result)
    }

    /// Helper methods to extract column values with type handling
    fn get_mysql_column_value(
        row: &sqlx::mysql::MySqlRow,
        index: usize,
        column_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Try different types in order of likelihood
        if let Ok(value) = row.try_get::<Option<String>, _>(index) {
            return Ok(value.unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<i64>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<f64>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<bool>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        // If all else fails, try to get the raw value as bytes and convert to string
        match row.try_get::<Option<Vec<u8>>, _>(index) {
            Ok(Some(bytes)) => String::from_utf8(bytes).map_err(|e| {
                format!("UTF-8 conversion error for column '{}': {}", column_name, e).into()
            }),
            Ok(None) => Ok(String::new()),
            Err(_) => Ok(String::new()), // Default to empty string if we can't extract the value
        }
    }

    /// Extracts a column value from a PostgreSQL row with type handling
    fn get_postgres_column_value(
        row: &sqlx::postgres::PgRow,
        index: usize,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Try different types in order of likelihood
        if let Ok(value) = row.try_get::<Option<String>, _>(index) {
            return Ok(value.unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<i64>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<f64>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<bool>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        // PostgreSQL specific types
        if let Ok(value) = row.try_get::<Option<i32>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        // Default to empty string if we can't extract the value
        Ok(String::new())
    }

    /// Extracts a column value from a SQLite row with type handling
    fn get_sqlite_column_value(
        row: &sqlx::sqlite::SqliteRow,
        index: usize,
        column_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // SQLite is more flexible with types, try common ones
        if let Ok(value) = row.try_get::<Option<String>, _>(index) {
            return Ok(value.unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<i64>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<f64>, _>(index) {
            return Ok(value.map(|v| v.to_string()).unwrap_or_default());
        }

        if let Ok(value) = row.try_get::<Option<Vec<u8>>, _>(index) {
            return match value {
                Some(bytes) => String::from_utf8(bytes).map_err(|e| {
                    format!("UTF-8 conversion error for column '{}': {}", column_name, e).into()
                }),
                None => Ok(String::new()),
            };
        }

        // Default to empty string if we can't extract the value
        Ok(String::new())
    }
}
