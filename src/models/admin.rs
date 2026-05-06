use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
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
    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let argon2 = Argon2::default();
        match PasswordHash::new(&self.password_hash) {
            Ok(parsed_hash) => argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(_) => false,
        }
    }

    pub async fn get_by_email(
        pool: &sqlx::PgPool,
        email: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, first_name, last_name, email, password_hash, phone, profile_image_url, role as "role!: AdminRole", is_active as "is_active!", last_login as "last_login?: OffsetDateTime", created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
            FROM admins
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, first_name, last_name, email, password_hash, phone, profile_image_url, role as "role!: AdminRole", is_active as "is_active!", last_login as "last_login?: OffsetDateTime", created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
            FROM admins
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }
}

pub fn get_all_system_users() -> Vec<Admin> {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

    // This function would typically fetch data from a database
    vec![
        Admin {
            id: 1,
            first_name: "Alice".to_string(),
            last_name: "Smith".to_string(),
            email: "alice.smith@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "123-456-7890".to_string(),
            role: AdminRole::Admin,
            is_active: true,
            last_login: None,
            created_at: now,
            updated_at: now,
            profile_image_url: None,
        },
        Admin {
            id: 2,
            first_name: "Bob".to_string(),
            last_name: "Johnson".to_string(),
            email: "bob.johnson@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "987-654-3210".to_string(),
            role: AdminRole::Manager,
            is_active: true,
            last_login: None,
            created_at: now,
            updated_at: now,
            profile_image_url: None,
        },
    ]
}
