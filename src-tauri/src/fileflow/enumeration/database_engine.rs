use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    MariaDB,
    MySQL,
    Postgres,
    SQLite,
}

// impl DatabaseEngine {
// pub fn as_str(&self) -> &'static str {
//     match self {
//         DatabaseEngine::MariaDB => "mariadb",
//         DatabaseEngine::MySQL => "mysql",
//         DatabaseEngine::Postgres => "postgres",
//         DatabaseEngine::SQLite => "sqlite",
//     }
// }
// }
