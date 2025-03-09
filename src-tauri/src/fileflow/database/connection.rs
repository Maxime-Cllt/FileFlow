use crate::fileflow::stuct::db_config::DbConfig;
use crate::fileflow::utils::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
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
        let connection_str: String = Self::get_connection_url(config)?;

        let connection_enum = match config.db_driver.as_str() {
            POSTGRES => {
                let pool: Pool<Postgres> = PgPool::connect(&connection_str).await?;
                ConnectionEnum::Postgres(pool)
            }
            MYSQL | MARIADB => {
                let pool: Pool<MySql> = MySqlPool::connect(&connection_str).await?;
                ConnectionEnum::MySQL(pool)
            }
            SQLITE => {
                let pool: Pool<Sqlite> = SqlitePool::connect(&connection_str).await?;
                ConnectionEnum::SQLite(pool)
            }
            _ => return Err(Error::Configuration("Unsupported database driver".into())),
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

    pub fn get_pool(&self) -> Result<ConnectionEnum, Error> {
        match &self.connection {
            ConnectionEnum::Postgres(pool) => Ok(ConnectionEnum::Postgres(pool.clone())),
            ConnectionEnum::MySQL(pool) => Ok(ConnectionEnum::MySQL(pool.clone())),
            ConnectionEnum::SQLite(pool) => Ok(ConnectionEnum::SQLite(pool.clone())),
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

    pub(crate) fn get_connection_url(config: &DbConfig) -> Result<String, Error> {
        match config.db_driver.as_str() {
            POSTGRES => Ok(format!(
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
            )),
            MYSQL | MARIADB => Ok(format!(
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
            )),
            SQLITE => Ok(config.sqlite_file_path.clone()),
            _ => Err(Error::Protocol(format!(
                "Unsupported database driver: {}",
                config.db_driver
            ))),
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
