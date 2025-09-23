use crate::fileflow::enums::database_engine::DatabaseEngine;
use crate::fileflow::enums::insertion_type::InsertionType;
use serde::Deserialize;

/// Struct for inserting data into a database
#[derive(Deserialize)]
pub struct InsertConfig {
    pub file_path: String,
    pub table_name: String,
    pub mode: InsertionType,
    pub db_driver: DatabaseEngine,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert_config() {
        let config = InsertConfig {
            file_path: "file_path".into(),
            table_name: "table_name".into(),
            mode: InsertionType::Fast,
            db_driver: DatabaseEngine::Postgres,
        };

        assert_eq!(config.db_driver, DatabaseEngine::Postgres);
        assert_eq!(config.file_path, "file_path");
        assert_eq!(config.table_name, "table_name");
        assert_eq!(config.mode, InsertionType::Fast);
    }

    #[tokio::test]
    async fn test_insert_config_deserialization() {
        let json_data = r#"
        {
            "file_path": "file_path",
            "table_name": "table_name",
            "mode": "optimized",
            "db_driver": "mysql"
        }
        "#;

        let deserialized: InsertConfig = serde_json::from_str(json_data).unwrap();

        assert_eq!(deserialized.db_driver, DatabaseEngine::MySQL);
        assert_eq!(deserialized.file_path, "file_path");
        assert_eq!(deserialized.table_name, "table_name");
        assert_eq!(deserialized.mode, InsertionType::Optimized);
    }
}
