
use sqlx::{PgPool, MySqlPool, Error}; // Assuming sqlx is being used for database connections
pub(crate) mod db_config;
pub(crate) mod insert_config;
pub(crate) mod save_config;

use db_config::DbConfig;

pub enum DatabaseConnection {
    Postgres(PgPool),
    MySQL(MySqlPool),
}

impl DatabaseConnection {
    // Connect to the appropriate database based on the database_connection
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
            "mysql" => {
                let pool = MySqlPool::connect(&format!(
                    "mysql://{}:{}@{}:{}/{}",
                    config.username, config.password, config.db_host, config.port, config.db_name
                ))
                    .await?;
                Ok(DatabaseConnection::MySQL(pool))
            }
            _ => Err(Error::Protocol(format!(
                "Unsupported database driver: {}",
                config.db_driver
            ))),
        }
    }

    // Query method for later usage
    pub async fn query(&self, query: &str) -> Result<(), Error> {
        match self {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            DatabaseConnection::MySQL(pool) => {
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
