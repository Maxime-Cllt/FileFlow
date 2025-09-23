use serde::{Deserialize, Serialize};

/// Defines the type of insertion to be used in database operations.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum InsertionType {
    #[default]
    Fast,
    Optimized, // Optimize column length based on data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insertion_type() {
        let insertion_type = InsertionType::Optimized;
        assert_eq!(insertion_type, InsertionType::Optimized);
    }

    #[tokio::test]
    async fn test_insertion_type_fast_serialization() {
        let insertion_type = InsertionType::Fast;
        let serialized = serde_json::to_string(&insertion_type).unwrap();
        assert_eq!(serialized, "\"fast\"");
        let deserialized: InsertionType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, InsertionType::Fast);
    }

    #[tokio::test]
    async fn test_insertion_type_optimized_serialization() {
        let insertion_type = InsertionType::Optimized;
        let serialized = serde_json::to_string(&insertion_type).unwrap();
        assert_eq!(serialized, "\"optimized\"");
        let deserialized: InsertionType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, InsertionType::Optimized);
    }
}
