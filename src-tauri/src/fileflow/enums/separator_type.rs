use serde::{Deserialize, Serialize};

/// Enum representing different types of separators for CSV files.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SeparatorType {
    #[default]
    Comma,
    Semicolon,
    Space,
    Pipe,
}

impl SeparatorType {
    /// Returns the separator as an u8.
    pub const fn as_u8(&self) -> u8 {
        match self {
            SeparatorType::Comma => b',',
            SeparatorType::Semicolon => b';',
            SeparatorType::Space => b' ',
            SeparatorType::Pipe => b'|',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SeparatorType;

    #[tokio::test]
    async fn test_as_u8() {
        assert_eq!(SeparatorType::Comma.as_u8(), b',');
        assert_eq!(SeparatorType::Semicolon.as_u8(), b';');
        assert_eq!(SeparatorType::Space.as_u8(), b' ');
        assert_eq!(SeparatorType::Pipe.as_u8(), b'|');
    }

    #[tokio::test]
    async fn test_default() {
        let default_separator = SeparatorType::default();
        assert_eq!(default_separator, SeparatorType::Comma);
    }

    #[tokio::test]
    async fn test_deserialize() {
        let comma: SeparatorType = serde_json::from_str(r#""comma""#).unwrap();
        let semicolon: SeparatorType = serde_json::from_str(r#""semicolon""#).unwrap();
        let space: SeparatorType = serde_json::from_str(r#""space""#).unwrap();
        let pipe: SeparatorType = serde_json::from_str(r#""pipe""#).unwrap();

        assert_eq!(comma, SeparatorType::Comma);
        assert_eq!(semicolon, SeparatorType::Semicolon);
        assert_eq!(space, SeparatorType::Space);
        assert_eq!(pipe, SeparatorType::Pipe);
    }

    #[tokio::test]
    async fn test_serialize() {
        let comma = serde_json::to_string(&SeparatorType::Comma).unwrap();
        let semicolon = serde_json::to_string(&SeparatorType::Semicolon).unwrap();
        let space = serde_json::to_string(&SeparatorType::Space).unwrap();
        let pipe = serde_json::to_string(&SeparatorType::Pipe).unwrap();
        assert_eq!(comma, r#""comma""#);
        assert_eq!(semicolon, r#""semicolon""#);
        assert_eq!(space, r#""space""#);
        assert_eq!(pipe, r#""pipe""#);
    }

}
