//! GitHub repository cache database operations
//!
//! Implements stale-while-revalidate caching pattern.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Cache configuration
pub const CACHE_FRESH_MINUTES: i64 = 5;
pub const CACHE_STALE_MINUTES: i64 = 60;
pub const GITHUB_USERNAME: &str = "noperoni";

/// Cached GitHub repository from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GithubRepo {
    pub id: Uuid,
    pub github_id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub language: Option<String>,
    pub stargazers_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub topics: serde_json::Value,
    pub github_created_at: Option<DateTime<Utc>>,
    pub github_updated_at: Option<DateTime<Utc>>,
    pub github_pushed_at: Option<DateTime<Utc>>,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
}

/// GitHub cache metadata (singleton)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GithubCacheMetadata {
    pub id: i32,
    pub last_successful_fetch: Option<DateTime<Utc>>,
    pub last_fetch_attempt: Option<DateTime<Utc>>,
    pub fetch_error_count: i32,
    pub last_error_message: Option<String>,
    pub rate_limit_remaining: Option<i32>,
    pub rate_limit_reset: Option<DateTime<Utc>>,
}

/// Simplified repo for client display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubRepoDisplay {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub language: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub topics: Vec<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// GitHub API response structure
#[derive(Debug, Clone, Deserialize)]
pub struct GithubApiRepo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub language: Option<String>,
    pub stargazers_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    #[serde(default)]
    pub topics: Vec<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub pushed_at: Option<DateTime<Utc>>,
    pub fork: bool,
}

/// Check if cache is fresh (within CACHE_FRESH_MINUTES)
pub async fn is_cache_fresh(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let cutoff = Utc::now() - Duration::minutes(CACHE_FRESH_MINUTES);

    let result: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT COUNT(*) as count
        FROM github_repos_cache
        WHERE is_active = TRUE AND cached_at > $1
        "#,
    )
    .bind(cutoff)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|(c,)| c > 0).unwrap_or(false))
}

/// Check if cache has any valid data (even if stale)
pub async fn has_cached_data(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let stale_cutoff = Utc::now() - Duration::minutes(CACHE_STALE_MINUTES);

    let result: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT COUNT(*) as count
        FROM github_repos_cache
        WHERE is_active = TRUE AND cached_at > $1
        "#,
    )
    .bind(stale_cutoff)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|(c,)| c > 0).unwrap_or(false))
}

/// Get all cached repositories for display
pub async fn get_cached_repos(pool: &PgPool) -> Result<Vec<GithubRepoDisplay>, sqlx::Error> {
    let repos: Vec<GithubRepo> = sqlx::query_as(
        r#"
        SELECT id, github_id, name, full_name, description, html_url, language,
               stargazers_count, forks_count, open_issues_count, topics,
               github_created_at, github_updated_at, github_pushed_at,
               cached_at, expires_at, is_active
        FROM github_repos_cache
        WHERE is_active = TRUE
        ORDER BY stargazers_count DESC, github_pushed_at DESC NULLS LAST
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(repos
        .into_iter()
        .map(|r| GithubRepoDisplay {
            name: r.name,
            description: r.description,
            html_url: r.html_url,
            language: r.language,
            stars: r.stargazers_count,
            forks: r.forks_count,
            topics: serde_json::from_value(r.topics).unwrap_or_default(),
            updated_at: r.github_pushed_at,
        })
        .collect())
}

/// Get cache metadata
pub async fn get_cache_metadata(pool: &PgPool) -> Result<GithubCacheMetadata, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, last_successful_fetch, last_fetch_attempt, fetch_error_count,
               last_error_message, rate_limit_remaining, rate_limit_reset
        FROM github_cache_metadata
        WHERE id = 1
        "#,
    )
    .fetch_one(pool)
    .await
}

/// Update cache metadata after fetch attempt
pub async fn update_cache_metadata(
    pool: &PgPool,
    success: bool,
    error_message: Option<&str>,
    rate_limit_remaining: Option<i32>,
    rate_limit_reset: Option<DateTime<Utc>>,
) -> Result<(), sqlx::Error> {
    if success {
        sqlx::query(
            r#"
            UPDATE github_cache_metadata
            SET last_successful_fetch = NOW(),
                last_fetch_attempt = NOW(),
                fetch_error_count = 0,
                last_error_message = NULL,
                rate_limit_remaining = $1,
                rate_limit_reset = $2
            WHERE id = 1
            "#,
        )
        .bind(rate_limit_remaining)
        .bind(rate_limit_reset)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE github_cache_metadata
            SET last_fetch_attempt = NOW(),
                fetch_error_count = fetch_error_count + 1,
                last_error_message = $1,
                rate_limit_remaining = $2,
                rate_limit_reset = $3
            WHERE id = 1
            "#,
        )
        .bind(error_message)
        .bind(rate_limit_remaining)
        .bind(rate_limit_reset)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Upsert repositories from GitHub API response
pub async fn upsert_repos(pool: &PgPool, repos: Vec<GithubApiRepo>) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    let expires = now + Duration::minutes(CACHE_FRESH_MINUTES);

    // Mark repos not in the new list as inactive
    let github_ids: Vec<i64> = repos.iter().map(|r| r.id).collect();

    sqlx::query(
        r#"
        UPDATE github_repos_cache
        SET is_active = FALSE
        WHERE github_id != ALL($1)
        "#,
    )
    .bind(&github_ids)
    .execute(pool)
    .await?;

    // Upsert each repo
    for repo in repos {
        sqlx::query(
            r#"
            INSERT INTO github_repos_cache (
                github_id, name, full_name, description, html_url, language,
                stargazers_count, forks_count, open_issues_count, topics,
                github_created_at, github_updated_at, github_pushed_at,
                cached_at, expires_at, is_active
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, TRUE)
            ON CONFLICT (github_id) DO UPDATE SET
                name = EXCLUDED.name,
                full_name = EXCLUDED.full_name,
                description = EXCLUDED.description,
                html_url = EXCLUDED.html_url,
                language = EXCLUDED.language,
                stargazers_count = EXCLUDED.stargazers_count,
                forks_count = EXCLUDED.forks_count,
                open_issues_count = EXCLUDED.open_issues_count,
                topics = EXCLUDED.topics,
                github_created_at = EXCLUDED.github_created_at,
                github_updated_at = EXCLUDED.github_updated_at,
                github_pushed_at = EXCLUDED.github_pushed_at,
                cached_at = EXCLUDED.cached_at,
                expires_at = EXCLUDED.expires_at,
                is_active = TRUE
            "#,
        )
        .bind(repo.id)
        .bind(&repo.name)
        .bind(&repo.full_name)
        .bind(&repo.description)
        .bind(&repo.html_url)
        .bind(&repo.language)
        .bind(repo.stargazers_count)
        .bind(repo.forks_count)
        .bind(repo.open_issues_count)
        .bind(serde_json::to_value(&repo.topics).unwrap_or_default())
        .bind(repo.created_at)
        .bind(repo.updated_at)
        .bind(repo.pushed_at)
        .bind(now)
        .bind(expires)
        .execute(pool)
        .await?;
    }

    Ok(())
}
