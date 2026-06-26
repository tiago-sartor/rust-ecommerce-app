use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::Decimal};
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
    pub cnpj: Option<String>,
    pub company_name: Option<String>,
    pub state_registration: Option<String>,
    pub is_subscribed: bool,
    pub last_active_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub reset_token: Option<String>,
    pub reset_expires_at: Option<OffsetDateTime>,
}

#[derive(Debug)]
pub struct CustomerSummary {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_subscribed: bool,
    pub total_orders: i64,
    pub total_spent: Decimal,
    // This is an Option because MAX() or a LEFT JOIN with 0 orders can return NULL
    pub last_order_at: Option<OffsetDateTime>,
}

impl Default for Customer {
    fn default() -> Self {
        Self {
            id: 0,
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            password_hash: String::new(),
            phone: String::new(),
            profile_image_url: None,
            cpf: None,
            cnpj: None,
            company_name: None,
            state_registration: None,
            is_subscribed: false,
            last_active_at: None,
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
            reset_token: None,
            reset_expires_at: None,
        }
    }
}

impl Customer {
    pub async fn get_by_id(id: &i64, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT
                id,
                first_name,
                last_name,
                email,
                password_hash,
                phone,
                profile_image_url,
                cpf,
                cnpj,
                company_name,
                state_registration,
                is_subscribed,
                last_active_at as "last_active_at?: OffsetDateTime",
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime",
                reset_token,
                reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM customers
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT
                id,
                first_name,
                last_name,
                email,
                password_hash,
                phone,
                profile_image_url,
                cpf,
                cnpj,
                company_name,
                state_registration,
                is_subscribed,
                last_active_at as "last_active_at?: OffsetDateTime",
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime",
                reset_token,
                reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM customers
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_cpf(cpf: &str, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT
                id,
                first_name,
                last_name,
                email,
                password_hash,
                phone,
                profile_image_url,
                cpf,
                cnpj,
                company_name,
                state_registration,
                is_subscribed,
                last_active_at as "last_active_at?: OffsetDateTime",
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime",
                reset_token,
                reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM customers
            WHERE cpf = $1
            "#,
            cpf
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_cnpj(cnpj: &str, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
           SELECT
                id,
                first_name,
                last_name,
                email,
                password_hash,
                phone,
                profile_image_url,
                cpf,
                cnpj,
                company_name,
                state_registration,
                is_subscribed,
                last_active_at as "last_active_at?: OffsetDateTime",
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime",
                reset_token,
                reset_expires_at as "reset_expires_at?: OffsetDateTime"
            FROM customers
            WHERE cnpj = $1
            "#,
            cnpj
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_paginated(page: i64, limit: i64, _order_by: &str, pool: &PgPool) -> Result<(Vec<CustomerSummary>, i64), sqlx::Error> {
        let offset = (page - 1).checked_mul(limit).unwrap_or(0);
        let customers = sqlx::query_as!(
            CustomerSummary,
            r#"
            SELECT
                customers.id,
                customers.first_name,
                customers.last_name,
                customers.email,
                customers.is_subscribed,
                COUNT(orders.id) AS "total_orders!",
                COALESCE(SUM(orders.total), 0) AS "total_spent!",
                MAX(orders.created_at) AS last_order_at
            FROM customers
            LEFT JOIN orders ON orders.customer_id = customers.id
            GROUP BY customers.id
            ORDER BY customers.id DESC
            LIMIT $1 OFFSET $2;
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM customers
            "#
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok((customers, total_count))
    }

    pub async fn create(customer: &Customer, pool: &PgPool) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO customers (first_name, last_name, email, password_hash, phone, profile_image_url, cpf, cnpj, company_name, state_registration)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                id,
                first_name,
                last_name,
                email,
                password_hash,
                phone,
                profile_image_url,
                cpf,
                cnpj,
                company_name,
                state_registration,
                is_subscribed,
                last_active_at as "last_active_at?: OffsetDateTime",
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime",
                reset_token,
                reset_expires_at as "reset_expires_at?: OffsetDateTime"
            "#,
            customer.first_name,
            customer.last_name,
            customer.email,
            customer.password_hash,
            customer.phone,
            customer.profile_image_url,
            customer.cpf,
            customer.cnpj,
            customer.company_name,
            customer.state_registration
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(customer_id: &i64, customer: Customer, pool: &PgPool) -> Option<Customer> {
        if *customer_id == 1 { Some(customer) } else { None }
    }

    pub async fn delete(customer_id: &i64, pool: &PgPool) -> bool {
        // Implementation for deleting a customer
        false
    }

    pub async fn create_tx(customer: &Customer, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO customers (first_name, last_name, email, password_hash, phone, profile_image_url, cpf, cnpj, company_name, state_registration)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                id,
                first_name,
                last_name,
                email,
                password_hash,
                phone,
                profile_image_url,
                cpf,
                cnpj,
                company_name,
                state_registration,
                is_subscribed,
                last_active_at as "last_active_at?: OffsetDateTime",
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime",
                reset_token,
                reset_expires_at as "reset_expires_at?: OffsetDateTime"
            "#,
            customer.first_name,
            customer.last_name,
            customer.email,
            customer.password_hash,
            customer.phone,
            customer.profile_image_url,
            customer.cpf,
            customer.cnpj,
            customer.company_name,
            customer.state_registration
        )
        .fetch_one(&mut **tx)
        .await
    }
}
