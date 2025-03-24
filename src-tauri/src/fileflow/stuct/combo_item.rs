use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
pub struct ComboItem {
    pub label: String,
    pub value: String,
}
