use sqlx::{Error, MySqlPool, PgPool, SqlitePool};

use crate::fileflow::stuct::db_config::DbConfig;

pub enum DatabaseConnection {
    Postgres(PgPool),
    MySQL(MySqlPool),
    SQLite(SqlitePool),
}

impl DatabaseConnection {
    pub async fn connect(config: &DbConfig) -> Result<DatabaseConnection, Error> {
        let connection_str: &String = &Self::get_connection_url(config)?;

        match config.db_driver.as_str() {
            "postgres" => PgPool::connect(&connection_str).await.map(DatabaseConnection::Postgres),
            "mysql" | "mariadb" => MySqlPool::connect(&connection_str).await.map(DatabaseConnection::MySQL),
            "sqlite" => SqlitePool::connect(&connection_str).await.map(DatabaseConnection::SQLite),
            _ => unreachable!(),
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

    pub fn get_connection_url(config: &DbConfig) -> Result<String, Error> {
        match config.db_driver.as_str() {
            "postgres" => Ok(format!(
                "postgres://{}{}@{}:{}/{}",
                config.username,
                if config.password.is_empty() { "".to_string() } else { format!(":{}", config.password) },
                config.db_host, config.port, config.db_name)
            ),
            "mysql" | "mariadb" => Ok(format!(
                "mysql://{}{}@{}:{}/{}",
                config.username,
                if config.password.is_empty() { "".to_string() } else { format!(":{}", config.password) },
                config.db_host, config.port, config.db_name
            )),
            "sqlite" => Ok(config.sqlite_file_path.clone()),
            _ => Err(Error::Protocol(format!("Unsupported database driver: {}", config.db_driver).into())),
        }
    }

    // Drop the connection
    pub fn disconnect(self) {
        drop(self);
    }
}
