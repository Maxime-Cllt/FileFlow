use serde::{Deserialize, Serialize};

/// A struct representing an item in a combo box with a label and a value.
#[derive(Deserialize, Clone, Serialize)]
pub struct ComboItem {
    pub label: String,
    pub value: String,
}

#[cfg(test)]
mod test {
    use super::ComboItem;

    #[tokio::test]
    async fn test_combo_item_serialization() {
        let item = ComboItem {
            label: "Option 1".to_string(),
            value: "value1".to_string(),
        };

        // Serialize the ComboItem to a JSON string
        let serialized = serde_json::to_string(&item).unwrap();
        assert_eq!(serialized, r#"{"label":"Option 1","value":"value1"}"#);

        // Deserialize the JSON string back to a ComboItem
        let deserialized: ComboItem = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.label, "Option 1");
        assert_eq!(deserialized.value, "value1");
    }

    #[tokio::test]
    async fn test_combo_item_clone() {
        let item = ComboItem {
            label: "Option 2".to_string(),
            value: "value2".to_string(),
        };
        let cloned_item = item.clone();
        assert_eq!(cloned_item.label, "Option 2");
        assert_eq!(cloned_item.value, "value2");
    }

    #[tokio::test]
    async fn test_deserialization() {
        let json_data = r#"{"label":"Test Label","value":"Test Value"}"#;
        let item: ComboItem = serde_json::from_str(json_data).unwrap();
        assert_eq!(item.label, "Test Label");
        assert_eq!(item.value, "Test Value");
    }
}
