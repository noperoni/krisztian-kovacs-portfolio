//! Contact form database operations
//!
//! Handles rate limiting and contact submission storage.

use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::NewContactSubmission;

/// Rate limit configuration
pub const RATE_LIMIT_WINDOW_MINUTES: i64 = 60;
pub const RATE_LIMIT_MAX_ATTEMPTS: i32 = 3;

/// Rate limit record from database
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ContactRateLimit {
    pub id: Uuid,
    pub ip_hash: String,
    pub attempt_count: i32,
    pub window_start: chrono::DateTime<Utc>,
    pub last_attempt: chrono::DateTime<Utc>,
}

/// Check and update rate limit for an IP hash.
/// Returns Ok(true) if the request is allowed, Ok(false) if rate limited.
pub async fn check_rate_limit(pool: &PgPool, ip_hash: &str) -> Result<bool, sqlx::Error> {
    let window_cutoff = Utc::now() - Duration::minutes(RATE_LIMIT_WINDOW_MINUTES);

    // Try to get existing rate limit record
    let existing: Option<ContactRateLimit> = sqlx::query_as(
        r#"
        SELECT id, ip_hash, attempt_count, window_start, last_attempt
        FROM contact_rate_limits
        WHERE ip_hash = $1
        "#,
    )
    .bind(ip_hash)
    .fetch_optional(pool)
    .await?;

    match existing {
        Some(record) => {
            if record.window_start < window_cutoff {
                // Window has expired - reset the counter
                sqlx::query(
                    r#"
                    UPDATE contact_rate_limits
                    SET attempt_count = 1, window_start = NOW(), last_attempt = NOW()
                    WHERE ip_hash = $1
                    "#,
                )
                .bind(ip_hash)
                .execute(pool)
                .await?;
                Ok(true)
            } else if record.attempt_count >= RATE_LIMIT_MAX_ATTEMPTS {
                // Rate limited - update last_attempt but don't increment
                sqlx::query(
                    r#"
                    UPDATE contact_rate_limits
                    SET last_attempt = NOW()
                    WHERE ip_hash = $1
                    "#,
                )
                .bind(ip_hash)
                .execute(pool)
                .await?;
                Ok(false)
            } else {
                // Allowed - increment counter
                sqlx::query(
                    r#"
                    UPDATE contact_rate_limits
                    SET attempt_count = attempt_count + 1, last_attempt = NOW()
                    WHERE ip_hash = $1
                    "#,
                )
                .bind(ip_hash)
                .execute(pool)
                .await?;
                Ok(true)
            }
        }
        None => {
            // First attempt - insert new record
            sqlx::query(
                r#"
                INSERT INTO contact_rate_limits (ip_hash, attempt_count, window_start, last_attempt)
                VALUES ($1, 1, NOW(), NOW())
                "#,
            )
            .bind(ip_hash)
            .execute(pool)
            .await?;
            Ok(true)
        }
    }
}

/// Insert a new contact submission into the database.
/// Returns the UUID of the newly created record.
pub async fn insert_contact(
    pool: &PgPool,
    submission: NewContactSubmission,
) -> Result<Uuid, sqlx::Error> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO contact_submissions
            (name, email, subject, message, honeypot_filled, ip_hash, user_agent, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending')
        RETURNING id
        "#,
    )
    .bind(&submission.name)
    .bind(&submission.email)
    .bind(&submission.subject)
    .bind(&submission.message)
    .bind(submission.honeypot_filled)
    .bind(&submission.ip_hash)
    .bind(&submission.user_agent)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Clean up old rate limit records (older than 24 hours).
/// Call this periodically to prevent table bloat.
pub async fn cleanup_rate_limits(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let cutoff = Utc::now() - Duration::hours(24);

    let result = sqlx::query(r#"DELETE FROM contact_rate_limits WHERE window_start < $1"#)
        .bind(cutoff)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
