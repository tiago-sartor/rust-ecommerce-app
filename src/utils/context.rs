use crate::middlewares::csrf::CsrfToken;
use crate::models::{admin::Admin, customer::Customer, product::Product};
use axum::extract::Form;
use std::collections::HashMap;

pub struct Context<P = (), D = ()> {
    pub admin: Admin,
    pub customer: Option<Customer>,
    pub product: Option<Product>,
    pub csrf_token: CsrfToken,
    pub payload: Form<P>,
    pub data: D,
    pub errors: HashMap<String, String>,
    pub flash_msg: HashMap<String, String>,
}

impl<P: Default, D: Default> Context<P, D> {
    pub fn new() -> Self {
        Context {
            admin: Admin::new(),
            customer: None,
            product: None,
            csrf_token: CsrfToken(String::new()),
            payload: Form(P::default()),
            data: D::default(),
            errors: HashMap::new(),
            flash_msg: HashMap::new(),
        }
    }
}
