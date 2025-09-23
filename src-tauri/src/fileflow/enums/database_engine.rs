use serde::{Deserialize, Serialize};

/// Enum representing supported database engines.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    MariaDB,
    MySQL,
    Postgres,
    SQLite,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_engine() {
        let db_engine = DatabaseEngine::Postgres;
        assert_eq!(db_engine, DatabaseEngine::Postgres);
    }

    #[tokio::test]
    async fn test_database_engine_serialization() {
        let db_engine = DatabaseEngine::MySQL;
        let serialized = serde_json::to_string(&db_engine).unwrap();
        assert_eq!(serialized, "\"mysql\"");
        let deserialized: DatabaseEngine = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, DatabaseEngine::MySQL);
    }
}
