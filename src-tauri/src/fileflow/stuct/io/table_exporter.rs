use crate::fileflow::stuct::io::batch_processor::BatchProcessor;
use crate::fileflow::stuct::database::database_connection::DatabaseConnection;
use crate::fileflow::stuct::config::download_config::DownloadConfig;
use csv::{Writer, WriterBuilder};
use std::fs::File;

/// Export utility for database tables to CSV
pub struct TableExporter;

impl TableExporter {
    pub const  BATCH_SIZE: u16 = 5_000;

    /// Exports a table's data to CSV using paginated queries
    pub async fn export_table(
        connection: &DatabaseConnection,
        download_config: &DownloadConfig,
        table_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Validate inputs
        Self::validate_inputs(table_name, download_config)?;

        // Setup CSV writer
        let file_path = Self::build_file_path(download_config, table_name);
        let mut csv_writer = Self::create_csv_writer(download_config, &file_path)?;

        // Export data in batches
        let mut batch_processor = BatchProcessor::new(connection, table_name);
        let mut is_first_batch = true;

        while let Some((columns, rows)) = batch_processor.next_batch().await? {
            if is_first_batch {
                Self::write_header(&mut csv_writer, &columns)?;
                is_first_batch = false;
            }
            Self::write_rows(&mut csv_writer, &rows)?;
        }

        Self::finalize_writer(csv_writer)?;
        Ok(())
    }

    // Private helper methods
    fn validate_inputs(
        table_name: &str,
        download_config: &DownloadConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if table_name.trim().is_empty() {
            return Err("Table name cannot be empty".into());
        }
        if download_config.location.trim().is_empty() {
            return Err("Download location cannot be empty".into());
        }
        Ok(())
    }

    fn build_file_path(download_config: &DownloadConfig, table_name: &str) -> String {
        format!("{}/{}_export.csv", download_config.location, table_name)
    }

    fn create_csv_writer(
        download_config: &DownloadConfig,
        file_path: &str,
    ) -> Result<Writer<File>, Box<dyn std::error::Error>> {
        WriterBuilder::new()
            .delimiter(download_config.separator.as_u8())
            .from_path(file_path)
            .map_err(|e| format!("Failed to create CSV writer: {}", e).into())
    }

    fn write_header(
        writer: &mut Writer<File>,
        columns: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        writer
            .write_record(columns)
            .map_err(|e| format!("Failed to write CSV header: {}", e).into())
    }

    fn write_rows(
        writer: &mut Writer<File>,
        rows: &[Vec<String>],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for row in rows {
            writer
                .write_record(row)
                .map_err(|e| format!("Failed to write CSV row: {}", e))?;
        }
        Ok(())
    }

    fn finalize_writer(mut writer: Writer<File>) -> Result<(), Box<dyn std::error::Error>> {
        writer
            .flush()
            .map_err(|e| format!("Failed to flush CSV writer: {}", e).into())
    }
}

#[cfg(test)]
mod tests {}