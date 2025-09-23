use sqlx::{MySqlPool, PgPool, SqlitePool};

/// A pool of database connections, supporting multiple database engines.
pub enum ConnectionPool {
    Postgres(PgPool),
    MySQL(MySqlPool),
    SQLite(SqlitePool),
}
