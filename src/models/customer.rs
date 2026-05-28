use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: String,
    pub profile_image_url: Option<String>,
    pub cpf: Option<String>,
    pub company_name: Option<String>,
    pub cnpj: Option<String>,
    pub state_registration: Option<String>,
    pub is_active: bool,
    pub last_login: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Customer {
    pub async fn get_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, first_name, last_name, email, password_hash, phone, profile_image_url, cpf, company_name, cnpj, state_registration, is_active as "is_active!", last_login as "last_login?: OffsetDateTime", created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
            FROM customers
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
            SELECT id, first_name, last_name, email, password_hash, phone, profile_image_url, cpf, company_name, cnpj, state_registration, is_active as "is_active!", last_login as "last_login?: OffsetDateTime", created_at as "created_at!: OffsetDateTime", updated_at as "updated_at!: OffsetDateTime"
            FROM customers
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }
}

pub async fn get_customer_by_cpf(cpf: &str) -> Option<Customer> {
    let now = OffsetDateTime::now_utc();

    // Simulate fetching customer from a database
    if cpf == "123.456.789-00" {
        Some(Customer {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "1234567890".to_string(),
            profile_image_url: None,
            cpf: Some("123.456.789-00".to_string()),
            company_name: None,
            cnpj: None,
            state_registration: None,
            is_active: true,
            last_login: None,
            created_at: now,
            updated_at: now,
        })
    } else {
        None
    }
}

pub async fn get_customer_by_cnpj(cnpj: &str) -> Option<Customer> {
    let now = OffsetDateTime::now_utc();

    // Simulate fetching customer from a database
    if cnpj == "12.345.678/0001-00" {
        Some(Customer {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "1234567890".to_string(),
            profile_image_url: None,
            cpf: None,
            company_name: Some("Awesome Company LLC".to_string()),
            cnpj: Some("12.345.678/0001-00".to_string()),
            state_registration: None,
            is_active: true,
            last_login: None,
            created_at: now,
            updated_at: now,
        })
    } else {
        None
    }
}

pub async fn create_customer(customer: Customer) -> Customer {
    // Simulate creating a customer in a database
    customer
}

pub async fn update_customer(customer_id: &i64, updated_customer: Customer) -> Option<Customer> {
    // Simulate updating a customer in a database
    if *customer_id == 1 { Some(updated_customer) } else { None }
}

pub async fn delete_customer(customer_id: &i64) -> bool {
    // Simulate deleting a customer from a database
    *customer_id == 1
}
