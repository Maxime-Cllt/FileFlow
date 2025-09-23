/// Rows returned from a database query, supporting multiple backends.
pub enum QueryRows {
    Postgres(Vec<sqlx::postgres::PgRow>),
    MySQL(Vec<sqlx::mysql::MySqlRow>),
    SQLite(Vec<sqlx::sqlite::SqliteRow>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_result_postgres() {
        // This is a placeholder test. In a real scenario, you would set up a test database.
        let rows = vec![];
        let result = QueryRows::Postgres(rows);
        match result {
            QueryRows::Postgres(r) => assert!(r.is_empty()),
            _ => panic!("Expected Postgres variant"),
        }
    }

    #[tokio::test]
    async fn test_query_result_mysql() {
        // This is a placeholder test. In a real scenario, you would set up a test database.
        let rows = vec![];
        let result = QueryRows::MySQL(rows);
        match result {
            QueryRows::MySQL(r) => assert!(r.is_empty()),
            _ => panic!("Expected MySQL variant"),
        }
    }

    #[tokio::test]
    async fn test_query_result_sqlite() {
        // This is a placeholder test. In a real scenario, you would set up a test database.
        let rows = vec![];
        let result = QueryRows::SQLite(rows);
        match result {
            QueryRows::SQLite(r) => assert!(r.is_empty()),
            _ => panic!("Expected SQLite variant"),
        }
    }
}
