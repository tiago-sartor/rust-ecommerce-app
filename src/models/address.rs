use crate::utils::BrazilianStates;
use time::OffsetDateTime;

pub struct Address {
    pub id: i64,
    pub customer_id: i64,
    pub street: String,
    pub number: Option<i32>,
    pub complement: String,
    pub neighborhood: String,
    pub city: String,
    pub state: Option<BrazilianStates>,
    pub postcode: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Default for Address {
    fn default() -> Self {
        Self {
            id: 0,
            customer_id: 0,
            street: String::new(),
            number: None,
            complement: String::new(),
            neighborhood: String::new(),
            city: String::new(),
            state: None,
            postcode: String::new(),
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl Address {
    pub async fn get_by_customer_id(customer_id: &i64, pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
       sqlx::query_as!(
            Self,
            r#"
            SELECT
                id,
                customer_id,
                street,
                number,
                complement,
                neighborhood,
                city,
                state as "state!: BrazilianStates",
                postcode,
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime"
            FROM addresses
            WHERE customer_id = $1
            "#,
            customer_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create_address(address: Address, pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        // Simulate creating an address in a database
        Ok(address)
    }

    pub async fn update_address(address_id: i64, address: Address, pool: &sqlx::PgPool) -> Option<Address> {
        // Simulate updating an address in a database
        Some(address)
    }

    pub async fn delete_address(address_id: i64, pool: &sqlx::PgPool) -> bool {
        // Simulate deleting an address from a database
        true
    }

    pub async fn create_tx(address: &Address, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO addresses (customer_id, street, number, complement, neighborhood, city, state, postcode)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id,
                customer_id,
                street,
                number,
                complement,
                neighborhood,
                city,
                state as "state?: BrazilianStates",
                postcode,
                created_at as "created_at!: OffsetDateTime",
                updated_at as "updated_at!: OffsetDateTime"
            "#,
            address.customer_id,
            address.street,
            address.number,
            address.complement,
            address.neighborhood,
            address.city,
            address.state as Option<BrazilianStates>,
            address.postcode
        )
        .fetch_one(&mut **tx)
        .await
    }
}
