//! GitHub API client for fetching repository data
//!
//! Uses a shared reqwest client from context for connection pooling.

use chrono::{DateTime, Utc};

use crate::db::github::GithubApiRepo;

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GithubClient {
    client: reqwest::Client,
    username: String,
}

pub struct FetchResult {
    pub repos: Vec<GithubApiRepo>,
    pub rate_limit_remaining: Option<i32>,
    pub rate_limit_reset: Option<DateTime<Utc>>,
}

#[derive(Debug, thiserror::Error)]
pub enum GithubError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Rate limited. Reset at: {0:?}")]
    RateLimited(Option<DateTime<Utc>>),

    #[error("GitHub API error: {status} - {message}")]
    ApiError { status: u16, message: String },
}

impl GithubClient {
    /// Create a new GitHub client using a shared HTTP client
    ///
    /// The HTTP client should be provided via Leptos context for connection reuse.
    pub fn new(client: reqwest::Client, username: &str) -> Self {
        Self {
            client,
            username: username.to_string(),
        }
    }

    /// Fetch all public repos for the configured user
    pub async fn fetch_repos(&self) -> Result<FetchResult, GithubError> {
        let url = format!(
            "{}/users/{}/repos?type=owner&sort=pushed&per_page=100",
            GITHUB_API_URL, self.username
        );

        let response = self.client.get(&url).send().await?;

        // Extract rate limit headers
        let rate_limit_remaining = response
            .headers()
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());

        let rate_limit_reset = response
            .headers()
            .get("x-ratelimit-reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp(ts, 0));

        let status = response.status();

        if status == reqwest::StatusCode::FORBIDDEN {
            // Check if rate limited
            if rate_limit_remaining == Some(0) {
                return Err(GithubError::RateLimited(rate_limit_reset));
            }
        }

        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(GithubError::ApiError {
                status: status.as_u16(),
                message,
            });
        }

        let repos: Vec<GithubApiRepo> = response.json().await?;

        // Filter out forks (show original repos only)
        let repos: Vec<GithubApiRepo> = repos.into_iter().filter(|r| !r.fork).collect();

        Ok(FetchResult {
            repos,
            rate_limit_remaining,
            rate_limit_reset,
        })
    }
}
