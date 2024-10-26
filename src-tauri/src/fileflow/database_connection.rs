use sqlx::{Error, MySqlPool, PgPool, Pool, Sqlite, SqlitePool};

use crate::fileflow::db_config::DbConfig;

pub enum DatabaseConnection {
    Postgres(PgPool),
    MySQL(MySqlPool),
    SQLite(SqlitePool),
}

impl DatabaseConnection {
    pub async fn connect(config: &DbConfig) -> Result<DatabaseConnection, Error> {
        match config.db_driver.as_str() {
            "postgres" => {
                let pool = PgPool::connect(&format!(
                    "postgres://{}:{}@{}:{}/{}",
                    config.username, config.password, config.db_host, config.port, config.db_name
                ))
                    .await?;
                Ok(DatabaseConnection::Postgres(pool))
            }
            "mysql" | "mariadb" => {
                let pool = MySqlPool::connect(&format!(
                    "mysql://{}:{}@{}:{}/{}",
                    config.username, config.password, config.db_host, config.port, config.db_name
                ))
                    .await?;
                Ok(DatabaseConnection::MySQL(pool))
            }
            "sqlite" => {
                let pool: Pool<Sqlite> = SqlitePool::connect(&config.sqlite_file_path).await?;
                Ok(DatabaseConnection::SQLite(pool))
            }
            _ => Err(Error::Protocol(format!(
                "Unsupported database driver: {}",
                config.db_driver
            ).into())),
        }
    }

    pub async fn query(&self, query: &str) -> Result<(), Error> {
        match self {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            DatabaseConnection::MySQL(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            DatabaseConnection::SQLite(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
        }
        Ok(())
    }

    // Drop the connection (disconnect)
    pub fn disconnect(self) {
        drop(self);
    }
}
