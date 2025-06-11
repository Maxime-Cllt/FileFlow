use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    MariaDB,
    MySQL,
    Postgres,
    SQLite,
}
