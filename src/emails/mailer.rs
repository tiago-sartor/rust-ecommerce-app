use hypertext::Renderable;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use thiserror::Error;
use time::OffsetDateTime;

use super::templates::{password_reset::password_reset_email, welcome::welcome_email};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailLog {
    pub id: i64,
    pub recipient: String,
    pub subject: String,
    pub status: String,
    pub response: String,
    pub sent_at: OffsetDateTime,
}

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("SMTP Transport Error: {0}")]
    TransportError(#[from] lettre::transport::smtp::Error),
    #[error("Email Building Error: {0}")]
    BuildError(#[from] lettre::error::Error),
    #[error("Address Parsing Error: {0}")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_address: String,
    pool: PgPool,
}

impl Mailer {
    pub fn new(pool: &PgPool) -> Result<Self, EmailError> {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
        let _smtp_port = env::var("SMTP_PORT").unwrap_or_else(|_| "2525".to_string());
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string());
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string());
        let from_address = env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@example.com".to_string());

        let creds = Credentials::new(smtp_username, smtp_password);

        let transport = match AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host) {
            Ok(builder) => builder.credentials(creds).build(),
            Err(e) => {
                tracing::error!("Failed to initialize SMTP transport: {e}");
                return Err(EmailError::TransportError(e));
            }
        };

        Ok(Self {
            transport,
            from_address,
            pool: pool.clone(),
        })
    }

    pub async fn get_logs_paginated(pool: &PgPool, page: i64, limit: i64) -> Result<(Vec<EmailLog>, i64), EmailError> {
        let offset = (page - 1) * limit;
        let logs = sqlx::query_as!(
            EmailLog,
            r#"
            SELECT id, recipient, subject, status, response as "response!: String", sent_at as "sent_at!: OffsetDateTime"
            FROM email_logs
            ORDER BY sent_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let count_record = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM email_logs
            "#
        )
        .fetch_one(pool)
        .await?;

        Ok((logs, count_record.count))
    }

    async fn log_email(&self, recipient: &str, subject: &str, status: &str, response: String) -> Result<(), EmailError> {
        sqlx::query!(
            r#"
            INSERT INTO email_logs (recipient, subject, status, response) VALUES ($1, $2, $3, $4)
            "#,
            recipient,
            subject,
            status,
            response
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn send_welcome_email(&self, to_email: &str, user_name: &str) -> Result<(), EmailError> {
        let template = welcome_email(user_name);
        let html_body = template.render().into_inner();
        let subject = "Welcome to our Store!";

        let email = Message::builder()
            .from(self.from_address.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        match self.transport.send(email).await {
            Ok(res) => {
                let response = serde_json::to_string(&res).unwrap_or_else(|_| format!("{:?}", res));
                self.log_email(to_email, subject, "sent", response).await?;
                Ok(())
            }
            Err(err) => {
                self.log_email(to_email, subject, "failed", err.to_string()).await?;
                Err(EmailError::TransportError(err))
            }
        }
    }

    pub async fn send_password_reset_email(&self, to_email: &str, reset_link: &str) -> Result<(), EmailError> {
        let template = password_reset_email(reset_link);
        let html_body = template.render().into_inner();
        let subject = "Password Reset Request";

        let email = Message::builder()
            .from(self.from_address.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        match self.transport.send(email).await {
            Ok(res) => {
                let response = serde_json::to_string(&res).unwrap_or_else(|_| format!("{:?}", res));
                self.log_email(to_email, subject, "sent", response).await?;
                Ok(())
            }
            Err(err) => {
                self.log_email(to_email, subject, "failed", err.to_string()).await?;
                Err(EmailError::TransportError(err))
            }
        }
    }
}
