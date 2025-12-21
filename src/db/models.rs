//! Database models for Portfolio v2
//!
//! These structs map to the PostgreSQL tables defined in migrations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Contact form submission status
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ContactStatus {
    #[default]
    Pending,
    Read,
    Replied,
    Spam,
}

/// Contact form submission from visitors
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContactSubmission {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,

    // Spam protection
    pub honeypot_filled: Option<bool>,
    pub ip_hash: Option<String>,
    pub user_agent: Option<String>,

    // Status
    pub status: String,
    pub read_at: Option<DateTime<Utc>>,
    pub replied_at: Option<DateTime<Utc>>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New contact submission (for INSERT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewContactSubmission {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
    pub honeypot_filled: bool,
    pub ip_hash: Option<String>,
    pub user_agent: Option<String>,
}

/// Page view record (privacy-first analytics)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PageView {
    pub id: Uuid,

    // Page info
    pub path: String,
    pub title: Option<String>,

    // Anonymized visitor info
    pub session_hash: Option<String>,
    pub referrer: Option<String>,
    pub referrer_domain: Option<String>,

    // Device info (non-identifying)
    pub device_type: Option<String>,
    pub browser_family: Option<String>,
    pub os_family: Option<String>,
    pub country_code: Option<String>,

    // Engagement
    pub time_on_page_seconds: Option<i32>,
    pub scroll_depth_percent: Option<i32>,

    // Timestamp
    pub viewed_at: DateTime<Utc>,
}

/// New page view (for INSERT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPageView {
    pub path: String,
    pub title: Option<String>,
    pub session_hash: Option<String>,
    pub referrer: Option<String>,
    pub referrer_domain: Option<String>,
    pub device_type: Option<String>,
    pub browser_family: Option<String>,
    pub os_family: Option<String>,
    pub country_code: Option<String>,
}

/// Analytics event (button clicks, interactions)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AnalyticsEvent {
    pub id: Uuid,

    // Event info
    pub event_name: String,
    pub event_category: Option<String>,
    pub event_data: Option<serde_json::Value>,

    // Context
    pub page_path: Option<String>,
    pub session_hash: Option<String>,

    // Timestamp
    pub occurred_at: DateTime<Utc>,
}

/// New analytics event (for INSERT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnalyticsEvent {
    pub event_name: String,
    pub event_category: Option<String>,
    pub event_data: Option<serde_json::Value>,
    pub page_path: Option<String>,
    pub session_hash: Option<String>,
}
