use crate::fileflow::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use crate::fileflow::stuct::db_config::DbConfig;
use sqlx::{Error, MySql, MySqlPool, PgPool, Pool, Postgres, Sqlite, SqlitePool};

pub enum DatabaseConnectionEnum {
    Postgres(PgPool),
    MySQL(MySqlPool),
    SQLite(SqlitePool),
}

pub struct DatabaseConnection {
    pub db_config: DbConfig,
    pub connection: DatabaseConnectionEnum,
}

impl DatabaseConnection {
    pub async fn connect(config: &DbConfig) -> Result<DatabaseConnection, Error> {
        let connection_str: String = Self::get_connection_url(config)?;

        let connection_enum = match config.db_driver.as_str() {
            POSTGRES => {
                let pool: Pool<Postgres> = PgPool::connect(&connection_str).await?;
                DatabaseConnectionEnum::Postgres(pool)
            }
            MYSQL | MARIADB => {
                let pool: Pool<MySql> = MySqlPool::connect(&connection_str).await?;
                DatabaseConnectionEnum::MySQL(pool)
            }
            SQLITE => {
                let pool: Pool<Sqlite> = SqlitePool::connect(&connection_str).await?;
                DatabaseConnectionEnum::SQLite(pool)
            }
            _ => return Err(Error::Configuration("Unsupported database driver".into())),
        };

        Ok(DatabaseConnection {
            db_config: config.clone(),
            connection: connection_enum,
        })
    }

    pub fn get_db_config(&self) -> &DbConfig {
        &self.db_config
    }

    pub async fn query(&self, query: &str) -> Result<(), Error> {
        match &self.connection {
            DatabaseConnectionEnum::Postgres(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            DatabaseConnectionEnum::MySQL(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            DatabaseConnectionEnum::SQLite(pool) => {
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
                    "".to_string()
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
                    "".to_string()
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

    // Drop the connection
    pub fn disconnect(&self) {
        match &self.connection {
            DatabaseConnectionEnum::Postgres(pool) => {
                drop(pool.close());
            }
            DatabaseConnectionEnum::MySQL(pool) => {
                drop(pool.close());
            }
            DatabaseConnectionEnum::SQLite(pool) => {
                drop(pool.close());
            }
        }
    }
}
