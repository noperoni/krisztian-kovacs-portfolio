//! Contact form server function
//!
//! Handles form submission with validation, rate limiting, storage, and email.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Contact form input from the client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormInput {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
    pub website: String, // Honeypot field - should be empty for real users
}

/// Contact form submission result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactResult {
    pub success: bool,
    pub message_key: String, // i18n key for the message
}

/// Submit a contact form
#[server]
pub async fn submit_contact(input: ContactFormInput) -> Result<ContactResult, ServerFnError> {
    use crate::db::{check_rate_limit, insert_contact, NewContactSubmission};
    use crate::email::{send_contact_notification, EmailConfig};
    use axum::http::request::Parts;
    use leptos_axum::extract;
    use sha2::{Digest, Sha256};

    // Extract request info for IP-based rate limiting
    let parts: Parts = extract().await?;

    // Get IP address (check X-Forwarded-For for proxied requests)
    let ip = parts
        .headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Hash the IP for privacy (never store raw IPs)
    let salt = std::env::var("CONTACT_SALT")
        .unwrap_or_else(|_| "portfolio-contact-salt-2025".to_string());
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    hasher.update(salt.as_bytes());
    let ip_hash = format!("{:x}", hasher.finalize());

    // Get user agent for logging
    let user_agent = parts
        .headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Get database pool from context
    let pool = expect_context::<sqlx::PgPool>();

    // Check honeypot - if filled, it's a bot. Accept silently but flag as spam.
    if !input.website.is_empty() {
        let submission = NewContactSubmission {
            name: input.name,
            email: input.email,
            subject: if input.subject.is_empty() {
                None
            } else {
                Some(input.subject)
            },
            message: input.message,
            honeypot_filled: true,
            ip_hash: Some(ip_hash),
            user_agent,
        };
        let _ = insert_contact(&pool, submission).await;

        // Return success to not reveal detection
        return Ok(ContactResult {
            success: true,
            message_key: "contact_success".to_string(),
        });
    }

    // Validate name
    let name = input.name.trim();
    if name.is_empty() || name.len() > 255 {
        return Ok(ContactResult {
            success: false,
            message_key: "contact_error_name".to_string(),
        });
    }

    // Validate email
    // Security: reject newlines to prevent email header injection attacks
    let email = input.email.trim();
    if email.is_empty()
        || !email.contains('@')
        || email.len() > 255
        || email.contains('\n')
        || email.contains('\r')
    {
        return Ok(ContactResult {
            success: false,
            message_key: "contact_error_email".to_string(),
        });
    }

    // Validate message
    let message = input.message.trim();
    if message.is_empty() || message.len() > 5000 {
        return Ok(ContactResult {
            success: false,
            message_key: "contact_error_message".to_string(),
        });
    }

    // Validate subject (optional but has max length)
    let subject = input.subject.trim();
    if subject.len() > 500 {
        return Ok(ContactResult {
            success: false,
            message_key: "contact_error_subject".to_string(),
        });
    }

    // Check rate limit
    match check_rate_limit(&pool, &ip_hash).await {
        Ok(true) => {} // Allowed
        Ok(false) => {
            return Ok(ContactResult {
                success: false,
                message_key: "contact_error_rate_limit".to_string(),
            });
        }
        Err(e) => {
            tracing::error!(?e, "Rate limit check failed");
            return Err(ServerFnError::new("Database error"));
        }
    }

    // Insert into database
    let submission = NewContactSubmission {
        name: name.to_string(),
        email: email.to_string(),
        subject: if subject.is_empty() {
            None
        } else {
            Some(subject.to_string())
        },
        message: message.to_string(),
        honeypot_filled: false,
        ip_hash: Some(ip_hash),
        user_agent,
    };

    if let Err(e) = insert_contact(&pool, submission).await {
        tracing::error!(?e, "Failed to insert contact");
        return Err(ServerFnError::new("Failed to save message"));
    }

    // Send email notification (don't fail the request if email fails)
    if let Some(config) = EmailConfig::from_env() {
        if let Err(e) = send_contact_notification(
            &config,
            name,
            email,
            if subject.is_empty() {
                None
            } else {
                Some(subject)
            },
            message,
        )
        .await
        {
            tracing::error!(?e, "Failed to send email notification");
            // Don't fail - message is saved in DB, email is a bonus
        }
    } else {
        tracing::warn!("Email not configured - skipping notification");
    }

    Ok(ContactResult {
        success: true,
        message_key: "contact_success".to_string(),
    })
}
