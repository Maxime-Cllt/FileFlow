use crate::fileflow::enums::separator_type::SeparatorType;
use serde::{Deserialize, Serialize};

/// Configuration for downloading data from database tables
#[derive(Deserialize, Serialize)]
pub struct DownloadConfig {
    pub table_name_list: Vec<String>,
    pub location: String,
    pub separator: SeparatorType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_config() {
        let config = DownloadConfig {
            table_name_list: vec!["table1".into(), "table2".into()],
            location: "location".into(),
            separator: SeparatorType::Comma,
        };

        assert_eq!(config.table_name_list.len(), 2);
        assert_eq!(config.table_name_list[0], "table1");
        assert_eq!(config.table_name_list[1], "table2");
        assert_eq!(config.location, "location");
        assert_eq!(config.separator, SeparatorType::Comma);
    }

    #[tokio::test]
    async fn test_download_config_serialization() {
        let config = DownloadConfig {
            table_name_list: vec!["table1".into(), "table2".into()],
            location: "location".into(),
            separator: SeparatorType::Semicolon,
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: DownloadConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.table_name_list.len(), 2);
        assert_eq!(deserialized.table_name_list[0], "table1");
        assert_eq!(deserialized.table_name_list[1], "table2");
        assert_eq!(deserialized.location, "location");
        assert_eq!(deserialized.separator, SeparatorType::Semicolon);
    }
}
