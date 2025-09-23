use crate::fileflow::stuct::database::database_connection::DatabaseConnection;
use crate::fileflow::stuct::io::row_extractor::RowExtractor;
use crate::fileflow::stuct::io::table_exporter::TableExporter;

/// Handles paginated database queries for table export
pub(crate) struct BatchProcessor<'a> {
    connection: &'a DatabaseConnection,
    base_query: String,
    offset: u32,
}

impl<'a> BatchProcessor<'a> {
    /// Creates a new BatchProcessor for the given table
    #[inline]
    pub(crate) fn new(connection: &'a DatabaseConnection, table_name: &str) -> Self {
        Self {
            connection,
            base_query: format!("SELECT * FROM {table_name}"),
            offset: 0,
        }
    }

    /// Fetches the next batch of rows from the database
    pub(crate) async fn next_batch(
        &mut self,
    ) -> Result<Option<(Vec<String>, Vec<Vec<String>>)>, Box<dyn std::error::Error>> {
        let query = format!(
            "{} LIMIT {} OFFSET {}",
            self.base_query,
            TableExporter::BATCH_SIZE,
            self.offset
        );

        let query_result = self.connection.query_many_with_result(&query).await?;
        let (columns, rows) = RowExtractor::extract_data(query_result)?;

        if rows.is_empty() {
            return Ok(None);
        }

        self.offset += u32::from(TableExporter::BATCH_SIZE);
        Ok(Some((columns, rows)))
    }
}
