use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::Decimal};
use std::collections::HashMap;
use time::OffsetDateTime;

pub struct Order {
    pub id: i64,
    pub customer_id: i64,
    pub status: OrderStatus,
    pub discount: Decimal,
    pub shipping_total: Decimal,
    pub total: Decimal,
    pub coupons: HashMap<String, HashMap<u32, String>>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum OrderStatus {
    #[default]
    Draft,
    PaymentPending,
    PaymentFailed,
    Paid,
    Processing,
    Shipped,
    Completed,
    Cancelled,
    PartiallyRefunded,
    Refunded,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            id: 0,
            customer_id: 0,
            status: OrderStatus::default(),
            discount: Decimal::default(),
            shipping_total: Decimal::default(),
            total: Decimal::default(),
            coupons: HashMap::new(),
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl Order {
    pub async fn get_paginated_by_customer_id(customer_id: &i64, page: &i64, limit: &i64, _order_by: &str, pool: &PgPool) -> Result<(Vec<Order>, i64), sqlx::Error> {
        Ok((vec![Self::default()], 15))
    }
}
