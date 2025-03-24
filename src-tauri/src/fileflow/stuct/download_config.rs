use crate::fileflow::enumeration::separator::SeparatorType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DownloadConfig {
    pub table_name: String,
    pub location: String,
    pub separator: SeparatorType,
}
