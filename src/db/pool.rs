//! Database connection pool management
//!
//! Provides PostgreSQL connection pooling via SQLx with 2025 best practices.

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Create a new database connection pool
///
/// Reads DATABASE_URL from environment (via dotenvy in main.rs)
///
/// Configuration follows 2025 best practices:
/// - `test_before_acquire(false)`: Cancellation-safe for async code
/// - `after_connect`: Sets application_name for debugging in pg_stat_activity
pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        // Cancellation-safe: don't test connection if acquire is cancelled
        .test_before_acquire(false)
        // Set application name for debugging (visible in pg_stat_activity)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET application_name = 'portfolio'")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await
}

/// Run embedded migrations
///
/// Uses SQLx's compile-time migration embedding
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
}
