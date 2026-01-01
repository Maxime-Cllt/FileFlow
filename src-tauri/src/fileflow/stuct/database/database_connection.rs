use crate::fileflow::enums::connection_type::ConnectionPool;
use crate::fileflow::enums::database_engine::DatabaseEngine;
use crate::fileflow::enums::query_rows::QueryRows;
use crate::fileflow::stuct::config::db_config::DatabaseConfig;
use sqlx::{Error, MySql, MySqlPool, PgPool, Pool, Postgres, Sqlite, SqlitePool};

/// Represents a connection to a database, supporting multiple database engines.
#[must_use]
pub struct DatabaseConnection {
    pub database_config: DatabaseConfig,
    pub engine: ConnectionPool,
}

impl DatabaseConnection {
    /// Establishes a connection to the database based on the provided configuration.
    /// Returns a Connection instance if successful, or an Error if the connection fails.
    pub async fn connect(config: &DatabaseConfig) -> Result<Self, Error> {
        let connection_str: String = Self::get_connection_url(config);

        let connection_enum: ConnectionPool = match config.db_driver {
            DatabaseEngine::Postgres => {
                let pool: Pool<Postgres> = PgPool::connect(&connection_str).await?;
                ConnectionPool::Postgres(pool)
            }
            DatabaseEngine::MariaDB | DatabaseEngine::MySQL => {
                let pool: Pool<MySql> = MySqlPool::connect(&connection_str).await?;
                ConnectionPool::MySQL(pool)
            }
            DatabaseEngine::SQLite => {
                let pool: Pool<Sqlite> = SqlitePool::connect(&connection_str).await?;
                ConnectionPool::SQLite(pool)
            }
        };

        Ok(Self {
            database_config: config.clone(),
            engine: connection_enum,
        })
    }

    /// Returns a reference to the database configuration.
    pub const fn get_db_config(&self) -> &DatabaseConfig {
        &self.database_config
    }

    /// Executes a query and returns the query result wrapped in a QueryResult enum.
    pub async fn query_many_with_result(&self, query: &str) -> Result<QueryRows, Error> {
        match &self.engine {
            ConnectionPool::Postgres(pool) => {
                let rows = sqlx::query(query).fetch_all(pool).await?;
                Ok(QueryRows::Postgres(rows))
            }
            ConnectionPool::MySQL(pool) => {
                let rows = sqlx::query(query).fetch_all(pool).await?;
                Ok(QueryRows::MySQL(rows))
            }
            ConnectionPool::SQLite(pool) => {
                let rows = sqlx::query(query).fetch_all(pool).await?;
                Ok(QueryRows::SQLite(rows))
            }
        }
    }

    /// Executes a query without returning any result.
    pub async fn query(&self, query: &str) -> Result<(), Error> {
        match &self.engine {
            ConnectionPool::Postgres(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            ConnectionPool::MySQL(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            ConnectionPool::SQLite(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
        }
        Ok(())
    }

    /// Constructs the database connection URL based on the provided configuration.
    pub fn get_connection_url(config: &DatabaseConfig) -> String {
        match config.db_driver {
            DatabaseEngine::Postgres => format!(
                "postgres://{}{}@{}:{}/{}",
                config.username,
                if config.password.is_empty() {
                    String::new()
                } else {
                    format!(":{}", config.password)
                },
                config.db_host,
                config.port,
                config.db_name
            ),
            DatabaseEngine::MySQL | DatabaseEngine::MariaDB => format!(
                "mysql://{}{}@{}:{}/{}",
                config.username,
                if config.password.is_empty() {
                    String::new()
                } else {
                    format!(":{}", config.password)
                },
                config.db_host,
                config.port,
                config.db_name
            ),
            DatabaseEngine::SQLite => config.sqlite_file_path.clone(),
        }
    }

    /// Closes the database connection.
    pub async fn disconnect(self) {
        match self.engine {
            ConnectionPool::Postgres(pool) => {
                let _ = pool.close().await;
            }
            ConnectionPool::MySQL(pool) => {
                let _ = pool.close().await;
            }
            ConnectionPool::SQLite(pool) => {
                let _ = pool.close().await;
            }
        }
    }
}
