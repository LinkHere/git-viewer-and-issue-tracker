use anyhow::Result;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use std::env;
use std::str::FromStr;
use std::time::Duration;

pub async fn init_db() -> Result<SqlitePool> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let connection_options = SqliteConnectOptions::from_str(&db_url)?
        .foreign_keys(true)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
