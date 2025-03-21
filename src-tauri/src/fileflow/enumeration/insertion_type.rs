use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum InsertionType {
    #[default]
    Fast,
    Optimized,
}
