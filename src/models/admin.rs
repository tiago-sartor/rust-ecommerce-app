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
    pub async fn get_by_email(pool: &sqlx::PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
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

    pub async fn get_by_id(pool: &sqlx::PgPool, id: &i64) -> Result<Option<Self>, sqlx::Error> {
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

    pub async fn get_by_reset_token(pool: &sqlx::PgPool, reset_token: &str) -> Result<Option<Self>, sqlx::Error> {
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

    pub async fn clear_reset_token(pool: &sqlx::PgPool, id: &i64) -> Result<(), sqlx::Error> {
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

    pub async fn update_password(pool: &sqlx::PgPool, id: &i64, password_hash: &str) -> Result<(), sqlx::Error> {
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
}
