//! Database storage initialization and configuration

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;
use std::time::Duration;

use crate::error::Result;

/// Initialize database connection pool with WAL mode
pub async fn init_db_pool(database_url: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)  // Enable WAL mode
        .busy_timeout(Duration::from_secs(30))
        .disable_statement_logging();  // Chain this method

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect_with(options)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_initialization() -> Result<()> {
        let pool = init_db_pool("sqlite::memory:").await?;
        assert!(pool.acquire().await.is_ok());
        Ok(())
    }
}
