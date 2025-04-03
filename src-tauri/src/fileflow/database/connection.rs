use crate::fileflow::enumeration::database_engine::DatabaseEngine;
use crate::fileflow::stuct::db_config::DbConfig;
use sqlx::{Error, MySql, MySqlPool, PgPool, Pool, Postgres, Sqlite, SqlitePool};

pub enum ConnectionEnum {
    Postgres(PgPool),
    MySQL(MySqlPool),
    SQLite(SqlitePool),
}

pub enum QueryResult {
    Postgres(Vec<sqlx::postgres::PgRow>),
    MySQL(Vec<sqlx::mysql::MySqlRow>),
    SQLite(Vec<sqlx::sqlite::SqliteRow>),
}

pub struct Connection {
    pub db_config: DbConfig,
    pub connection: ConnectionEnum,
}

impl Connection {
    pub async fn connect(config: &DbConfig) -> Result<Self, Error> {
        let connection_str: String = Self::get_connection_url(config);

        let connection_enum: ConnectionEnum = match config.db_driver {
            DatabaseEngine::Postgres => {
                let pool: Pool<Postgres> = PgPool::connect(&connection_str).await?;
                ConnectionEnum::Postgres(pool)
            }
            DatabaseEngine::MariaDB | DatabaseEngine::MySQL => {
                let pool: Pool<MySql> = MySqlPool::connect(&connection_str).await?;
                ConnectionEnum::MySQL(pool)
            }
            DatabaseEngine::SQLite => {
                let pool: Pool<Sqlite> = SqlitePool::connect(&connection_str).await?;
                ConnectionEnum::SQLite(pool)
            }
        };

        Ok(Self {
            db_config: config.clone(),
            connection: connection_enum,
        })
    }

    pub const fn get_db_config(&self) -> &DbConfig {
        &self.db_config
    }

    /// Executes a query and returns the query result wrapped in a QueryResult enum.
    pub async fn query_many_with_result(&self, query: &str) -> Result<QueryResult, Error> {
        match &self.connection {
            ConnectionEnum::Postgres(pool) => {
                let rows = sqlx::query(query).fetch_all(pool).await?;
                Ok(QueryResult::Postgres(rows))
            }
            ConnectionEnum::MySQL(pool) => {
                let rows = sqlx::query(query).fetch_all(pool).await?;
                Ok(QueryResult::MySQL(rows))
            }
            ConnectionEnum::SQLite(pool) => {
                let rows = sqlx::query(query).fetch_all(pool).await?;
                Ok(QueryResult::SQLite(rows))
            }
        }
    }

    pub async fn query(&self, query: &str) -> Result<(), Error> {
        match &self.connection {
            ConnectionEnum::Postgres(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            ConnectionEnum::MySQL(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            ConnectionEnum::SQLite(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
        }
        Ok(())
    }

    pub(crate) fn get_connection_url(config: &DbConfig) -> String {
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

    pub fn disconnect(&self) {
        match &self.connection {
            ConnectionEnum::Postgres(pool) => {
                drop(pool.close());
            }
            ConnectionEnum::MySQL(pool) => {
                drop(pool.close());
            }
            ConnectionEnum::SQLite(pool) => {
                drop(pool.close());
            }
        }
    }
}
