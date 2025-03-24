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
