//! Email notification service using lettre
//!
//! Sends email notifications for contact form submissions.

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

/// Email configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub from_address: String,
    pub to_address: String,
}

impl EmailConfig {
    /// Load email configuration from environment variables.
    /// Returns None if required variables are missing (email will be skipped).
    pub fn from_env() -> Option<Self> {
        Some(Self {
            smtp_host: std::env::var("SMTP_HOST").ok()?,
            smtp_port: std::env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587),
            smtp_user: std::env::var("SMTP_USER").ok()?,
            smtp_password: std::env::var("SMTP_PASSWORD").ok()?,
            from_address: std::env::var("SMTP_FROM")
                .unwrap_or_else(|_| "noreply@pilgrim.ovh".to_string()),
            to_address: std::env::var("CONTACT_EMAIL")
                .unwrap_or_else(|_| "kovacs@pilgrim.ovh".to_string()),
        })
    }
}

/// Send a contact form notification email.
///
/// # Arguments
/// * `config` - Email configuration
/// * `name` - Sender's name
/// * `email` - Sender's email (used as reply-to)
/// * `subject` - Optional subject line
/// * `message` - The message content
pub async fn send_contact_notification(
    config: &EmailConfig,
    name: &str,
    email: &str,
    subject: Option<&str>,
    message: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subject_line = format!(
        "[Portfolio Contact] {}",
        subject.unwrap_or("New message from portfolio")
    );

    let body = format!(
        r#"New contact form submission from your portfolio:

From: {} <{}>
Subject: {}

Message:
{}

---
This email was sent from the contact form at kovacs.pilgrim.ovh
"#,
        name,
        email,
        subject.unwrap_or("(no subject)"),
        message
    );

    let email_message = Message::builder()
        .from(config.from_address.parse()?)
        .reply_to(email.parse()?)
        .to(config.to_address.parse()?)
        .subject(subject_line)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;

    let creds = Credentials::new(config.smtp_user.clone(), config.smtp_password.clone());

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)?
            .port(config.smtp_port)
            .credentials(creds)
            .build();

    mailer.send(email_message).await?;

    Ok(())
}
