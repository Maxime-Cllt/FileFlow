use crate::fileflow::constants::{MARIADB, MYSQL, POSTGRES, SQLITE};
use crate::fileflow::stuct::db_config::DbConfig;
use sqlx::{Error, MySqlPool, PgPool, SqlitePool};

pub enum DatabaseConnection {
    Postgres(PgPool),
    MySQL(MySqlPool),
    SQLite(SqlitePool),
}

impl DatabaseConnection {
    pub async fn connect(config: &DbConfig) -> Result<DatabaseConnection, Error> {
        let connection_str: &String = &Self::get_connection_url(config)?;

        match config.db_driver.as_str() {
            POSTGRES => {
                let pool = PgPool::connect(connection_str).await?;
                Ok(DatabaseConnection::Postgres(pool))
            }
            MYSQL | MARIADB => {
                let pool = MySqlPool::connect(connection_str).await?;
                Ok(DatabaseConnection::MySQL(pool))
            }
            SQLITE => {
                let pool = SqlitePool::connect(connection_str).await?;
                Ok(DatabaseConnection::SQLite(pool))
            }
            _ => Err(Error::Configuration("Unsupported database driver".into())),
        }
    }

    pub async fn query(&self, query: &str) -> Result<(), Error> {
        match self {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query(query).execute(pool).await?;
                Ok(())
            }
            DatabaseConnection::MySQL(pool) => {
                sqlx::query(query).execute(pool).await?;
                Ok(())
            }
            DatabaseConnection::SQLite(pool) => {
                sqlx::query(query).execute(pool).await?;
                Ok(())
            }
        }
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
            _ => Err(Error::Protocol(
                format!("Unsupported database driver: {}", config.db_driver).into(),
            )),
        }
    }

    // Drop the connection
    pub fn disconnect(&self) {
        match self {
            DatabaseConnection::Postgres(pool) => {
                let _ = pool.close();
            }
            DatabaseConnection::MySQL(pool) => {
                let _ = pool.close();
            }
            DatabaseConnection::SQLite(pool) => {
                let _ = pool.close();
            }
        }
    }
}
