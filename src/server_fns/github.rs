//! GitHub repos server function with stale-while-revalidate caching

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Simplified repo for client display (shared between client and server)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubRepoDisplay {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub language: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub topics: Vec<String>,
    pub updated_at: Option<String>, // Formatted date string for display
}

/// Result of fetching GitHub repos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubReposResult {
    pub repos: Vec<GithubRepoDisplay>,
    pub is_stale: bool,
    pub last_updated: Option<String>,
}

/// Fetch GitHub repositories with SWR caching
#[server]
pub async fn get_github_repos() -> Result<GithubReposResult, ServerFnError> {
    use crate::db::github::{
        get_cache_metadata, get_cached_repos, has_cached_data, is_cache_fresh,
    };
    use chrono::Utc;

    let pool = expect_context::<sqlx::PgPool>();

    // Check cache freshness
    let cache_fresh = is_cache_fresh(&pool).await.unwrap_or(false);
    let has_data = has_cached_data(&pool).await.unwrap_or(false);

    // Helper to convert DB type to API type
    let convert_repos = |db_repos: Vec<crate::db::github::GithubRepoDisplay>| -> Vec<GithubRepoDisplay> {
        db_repos
            .into_iter()
            .map(|r| GithubRepoDisplay {
                name: r.name,
                description: r.description,
                html_url: r.html_url,
                language: r.language,
                stars: r.stars,
                forks: r.forks,
                topics: r.topics,
                updated_at: r.updated_at.map(|dt| dt.format("%b %d, %Y").to_string()),
            })
            .collect()
    };

    if cache_fresh {
        // Cache is fresh - return immediately
        let repos = get_cached_repos(&pool).await.map_err(|e| {
            leptos::logging::error!("Failed to get cached repos: {:?}", e);
            ServerFnError::new("Database error")
        })?;

        let metadata = get_cache_metadata(&pool).await.ok();

        return Ok(GithubReposResult {
            repos: convert_repos(repos),
            is_stale: false,
            last_updated: metadata
                .and_then(|m| m.last_successful_fetch)
                .map(|dt| dt.to_rfc3339()),
        });
    }

    if has_data {
        // Cache is stale but has data - return stale data and trigger background refresh
        let repos = get_cached_repos(&pool).await.map_err(|e| {
            leptos::logging::error!("Failed to get cached repos: {:?}", e);
            ServerFnError::new("Database error")
        })?;

        let metadata = get_cache_metadata(&pool).await.ok();

        // Spawn background refresh (fire-and-forget)
        let pool_clone = pool.clone();
        tokio::spawn(async move {
            if let Err(e) = refresh_github_cache(&pool_clone).await {
                leptos::logging::error!("Background GitHub refresh failed: {:?}", e);
            }
        });

        return Ok(GithubReposResult {
            repos: convert_repos(repos),
            is_stale: true,
            last_updated: metadata
                .and_then(|m| m.last_successful_fetch)
                .map(|dt| dt.to_rfc3339()),
        });
    }

    // No cached data - must fetch synchronously
    match refresh_github_cache(&pool).await {
        Ok(_) => {
            let repos = get_cached_repos(&pool).await.map_err(|e| {
                leptos::logging::error!("Failed to get repos after refresh: {:?}", e);
                ServerFnError::new("Database error")
            })?;

            Ok(GithubReposResult {
                repos: convert_repos(repos),
                is_stale: false,
                last_updated: Some(Utc::now().to_rfc3339()),
            })
        }
        Err(e) => {
            leptos::logging::error!("GitHub fetch failed with no cache: {:?}", e);
            Err(ServerFnError::new("Failed to fetch GitHub repos"))
        }
    }
}

/// Internal function to refresh the GitHub cache
#[cfg(feature = "ssr")]
async fn refresh_github_cache(
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use crate::db::github::{update_cache_metadata, upsert_repos, GITHUB_USERNAME};
    use crate::github::GithubClient;

    let client = GithubClient::new(GITHUB_USERNAME);

    match client.fetch_repos().await {
        Ok(result) => {
            // Update cache with new data
            upsert_repos(pool, result.repos).await?;

            // Update metadata
            update_cache_metadata(
                pool,
                true,
                None,
                result.rate_limit_remaining,
                result.rate_limit_reset,
            )
            .await?;

            leptos::logging::log!("GitHub cache refreshed successfully");
            Ok(())
        }
        Err(e) => {
            // Update metadata with error
            update_cache_metadata(pool, false, Some(&e.to_string()), None, None)
                .await
                .ok(); // Don't fail on metadata update failure

            Err(Box::new(e))
        }
    }
}
