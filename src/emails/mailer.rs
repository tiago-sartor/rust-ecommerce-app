use hypertext::Renderable;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;
use time::OffsetDateTime;

use crate::emails::templates::{customer::*, shared::*};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailLog {
    pub id: i64,
    pub recipient: String,
    pub subject: String,
    pub status: Status,
    pub html: String,
    pub response: String,
    pub sent_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "emaillogstatus", rename_all = "lowercase")]
pub enum Status {
    Sent,
    Failed,
}
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Sent => write!(f, "Sent"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}
impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "sent" => Ok(Status::Sent),
            "failed" => Ok(Status::Failed),
            _ => Err(()),
        }
    }
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

    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error),
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
        let _smtp_port = env::var("SMTP_PORT").unwrap_or_else(|_| "465".to_string());
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_else(|_| "".to_string());
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_else(|_| "".to_string());
        let from_address = env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@example.com".to_string());

        let creds = Credentials::new(smtp_username, smtp_password);

        let transport = match AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host) {
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

    async fn send_email(&self, to_email: &str, subject: &str, html_body: String) -> Result<(), EmailError> {
        let email = Message::builder()
            .from(self.from_address.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body.clone())?;

        match self.transport.send(email).await {
            Ok(res) => {
                let response = serde_json::to_string(&res).unwrap_or_else(|_| format!("{:?}", res));
                self.log_email(to_email, subject, Status::Sent, &html_body, response).await?;
                Ok(())
            }
            Err(err) => {
                self.log_email(to_email, subject, Status::Failed, &html_body, err.to_string()).await?;
                Err(EmailError::TransportError(err))
            }
        }
    }

    async fn log_email(&self, recipient: &str, subject: &str, status: Status, html: &str, response: String) -> Result<(), EmailError> {
        sqlx::query!(
            r#"
            INSERT INTO email_logs (recipient, subject, status, html, response) VALUES ($1, $2, $3, $4, $5)
            "#,
            recipient,
            subject,
            status as Status,
            html,
            response
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_logs_paginated(page: i64, limit: i64, filter_by: &Option<Status>, pool: &PgPool) -> Result<(Vec<EmailLog>, i64), EmailError> {
        let offset = (page - 1) * limit;
        let logs = sqlx::query_as!(
            EmailLog,
            r#"
            SELECT id, recipient, subject, status as "status: Status", html, response as "response!", sent_at as "sent_at!: OffsetDateTime"
            FROM email_logs
            WHERE ($3::emaillogstatus IS NULL OR status = $3)
            ORDER BY sent_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
            &filter_by as &Option<Status>
        )
        .fetch_all(pool)
        .await?;

        let count_record = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM email_logs
            WHERE ($1::emaillogstatus IS NULL OR status = $1)
            "#,
            &filter_by as &Option<Status>
        )
        .fetch_one(pool)
        .await?;

        Ok((logs, count_record.count))
    }

    pub async fn get_log_details(id: i64, pool: &PgPool) -> Result<serde_json::Value, EmailError> {
        let record = sqlx::query!(
            r#"
            SELECT html, response as "response!"
            FROM email_logs
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        /*
         * Attempt to parse the response as JSON. In the event of a failure, which happens when the string is just plain
         * text error message, the code inside the closure takes the original plain text string and converts it into
         * a serde_json::Value (specifically a JSON string type).
         */
        let response = serde_json::from_str(&record.response).unwrap_or_else(|_| serde_json::Value::String(record.response));

        Ok(serde_json::json!({ "html": record.html, "response": response }))
    }

    pub async fn resend_email(&self, id: i64) -> Result<(), EmailError> {
        let log = sqlx::query!(
            r#"
            SELECT recipient, subject, html
            FROM email_logs
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        self.send_email(&log.recipient, &log.subject, log.html).await?;
        Ok(())
    }

    pub async fn bulk_resend(&self, ids: &[i64]) -> Result<(), EmailError> {
        if ids.is_empty() {
            return Ok(());
        }

        let logs = sqlx::query!(
            r#"
            SELECT id, recipient, subject, html
            FROM email_logs
            WHERE id = ANY($1)
            "#,
            ids
        )
        .fetch_all(&self.pool)
        .await?;

        for log in logs {
            if let Err(e) = self.send_email(&log.recipient, &log.subject, log.html).await {
                tracing::error!("Failed to resend email ID #{}: {}", log.id, e);
            }
        }

        Ok(())
    }

    pub async fn delete_log(id: i64, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM email_logs
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn bulk_delete(ids: &[i64], pool: &PgPool) -> Result<(), sqlx::Error> {
        if ids.is_empty() {
            return Ok(());
        }

        sqlx::query!(
            r#"
            DELETE FROM email_logs
            WHERE id = ANY($1)
            "#,
            ids
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_logs(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            TRUNCATE TABLE email_logs RESTART IDENTITY;
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /**
     * Admin Emails
     */
    pub async fn send_password_reset_email(&self, to_email: &str, reset_link: &str) -> Result<(), EmailError> {
        let template = password_reset_email(reset_link);
        let html_body = template.render().into_inner();
        let subject = "Password Reset Request";

        self.send_email(to_email, subject, html_body).await?;
        Ok(())
    }

    /**
     * Customer Emails
     */
    pub async fn send_welcome_email(&self, to_email: &str, user_name: &str) -> Result<(), EmailError> {
        let template = welcome_email(user_name);
        let html_body = template.render().into_inner();
        let subject = "Welcome to our Store!";

        self.send_email(to_email, subject, html_body).await?;
        Ok(())
    }
}
