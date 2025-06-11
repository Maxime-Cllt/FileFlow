use serde::{Deserialize, Serialize};

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
    
    /// Returns the separator as a u8.
    pub const fn as_u8(&self) -> u8 {
        match self {
            SeparatorType::Comma => b',',
            SeparatorType::Semicolon => b';',
            SeparatorType::Space => b' ',
            SeparatorType::Pipe => b'|',
        }
    }
}
