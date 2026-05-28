use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: String,
    pub profile_image_url: Option<String>,
    pub role: AdminRole,
    pub is_active: bool,
    pub last_login: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub reset_token: Option<String>,
    pub reset_expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum AdminRole {
    Admin,
    Manager,
    Editor,
    Support,
}

impl Admin {
    pub fn new() -> Self {
        Admin {
            id: 0,
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            password_hash: String::new(),
            phone: String::new(),
            profile_image_url: None,
            role: AdminRole::Admin,
            is_active: false,
            last_login: None,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            reset_token: None,
            reset_expires_at: None,
        }
    }

    pub async fn get_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id,
                   first_name,
                   last_name,
                   email,
                   password_hash,
                   phone,
                   profile_image_url,
                   role as "role!: AdminRole",
                   is_active as "is_active!",
                   last_login as "last_login?: OffsetDateTime",
                   created_at as "created_at!: OffsetDateTime",
                   updated_at as "updated_at!: OffsetDateTime",
                   reset_token,
                   reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM admins
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_id(id: &i64, pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id,
                   first_name,
                   last_name,
                   email,
                   password_hash,
                   phone,
                   profile_image_url,
                   role as "role!: AdminRole",
                   is_active as "is_active!",
                   last_login as "last_login?: OffsetDateTime",
                   created_at as "created_at!: OffsetDateTime",
                   updated_at as "updated_at!: OffsetDateTime",
                   reset_token,
                   reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM admins
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_reset_token(reset_token: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id,
                   first_name,
                   last_name,
                   email,
                   password_hash,
                   phone,
                   profile_image_url,
                   role as "role!: AdminRole",
                   is_active as "is_active!",
                   last_login as "last_login?: OffsetDateTime",
                   created_at as "created_at!: OffsetDateTime",
                   updated_at as "updated_at!: OffsetDateTime",
                   reset_token,
                   reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM admins
            WHERE reset_token = $1
            "#,
            reset_token
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn update_reset_token(reset_token: &str, email: &str, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE admins
            SET reset_token = $1, reset_expires_at = NOW() + INTERVAL '1 hour'
            WHERE email = $2
            "#,
            reset_token,
            email
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn clear_reset_token(id: &i64, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE admins
            SET reset_token = NULL, reset_expires_at = NULL
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_password(id: &i64, password_hash: &str, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE admins
            SET password_hash = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            password_hash,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_all_system_users(pool: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id,
                   first_name,
                   last_name,
                   email,
                   password_hash,
                   phone,
                   profile_image_url,
                   role as "role!: AdminRole",
                   is_active as "is_active!",
                   last_login as "last_login?: OffsetDateTime",
                   created_at as "created_at!: OffsetDateTime",
                   updated_at as "updated_at!: OffsetDateTime",
                   reset_token,
                   reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM admins
            "#
        )
        .fetch_all(pool)
        .await
    }
}
