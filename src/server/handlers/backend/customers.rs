use crate::backend::{layouts::*, templates::*};
use crate::middlewares::csrf::CsrfToken;
use crate::models::{Address, Admin, Customer, CustomerSummary, Order, address};
use crate::server::handlers::backend::AddCustomerPayload;
use crate::utils::{AppError, BrazilianStates, Context, helpers, password};
use axum::{
    extract::{Extension, Form, Path, Query, State},
    response::{IntoResponse, Redirect},
};
use hypertext::Renderable;
use sqlx::PgPool;
use std::collections::HashMap;
use tower_sessions::Session;
use validator::Validate;

#[derive(Default)]
pub struct CustomersData {
    pub customers: Vec<CustomerSummary>,
    pub count: i64,
    pub page: i64,
    pub limit: i64,
}
pub struct CustomerDetailsData {
    pub orders: Vec<Order>,
}

pub async fn admin_customers(
    Extension(admin): Extension<Admin>,
    Extension(csrf_token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let page: i64 = params.get("page").and_then(|v| v.parse().ok()).map(|v: i64| v.clamp(1, 10_000)).unwrap_or(1);
    let limit: i64 = params.get("limit").and_then(|v| v.parse().ok()).map(|v: i64| v.clamp(10, 100)).unwrap_or(10);

    let (customers, count) = Customer::get_paginated(page, limit, "", &pool).await?;

    ctx.data = CustomersData { customers, count, page, limit };

    let template = admin_customers_template(&ctx);
    let page_scripts = vec!["admin-checkbox-selector", "admin-action-dropdown"];

    let html = admin_layout("Customers", template, &ctx, Some(page_scripts));
    Ok(html.render().into_response())
}

pub async fn admin_add_customer_get(
    Extension(admin): Extension<Admin>,
    Extension(csrf_token): Extension<CsrfToken>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    if let Some(flash_msg) = session.remove::<String>("customer_success_flash").await? {
        ctx.flash_msg.insert("success".to_string(), flash_msg);
    }

    let template = admin_add_customer_template(&ctx);
    let page_scripts = vec!["address-autocomplete"];
    let html = admin_layout("Add Customer", template, &ctx, Some(page_scripts));

    Ok(html.render().into_response())
}

pub async fn admin_add_customer_post(
    Extension(admin): Extension<Admin>,
    Extension(csrf_token): Extension<CsrfToken>,
    session: Session,
    State(pool): State<PgPool>,
    Form(payload): Form<AddCustomerPayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::<AddCustomerPayload, ()>::new();
    ctx.admin = Some(admin);
    ctx.payload = Form(payload);
    ctx.csrf_token = csrf_token;

    if let Err(e) = ctx.payload.validate() {
        for (field, errs) in e.field_errors() {
            for err in errs {
                if let Some(message) = &err.message {
                    ctx.errors.insert(field.to_string(), message.to_string());
                }
            }
        }
    } else {
        let is_cpf = ctx.payload.cpf_cnpj.len() == 11;
        let cpf = is_cpf.then(|| ctx.payload.cpf_cnpj.clone());
        let cnpj = (!is_cpf).then(|| ctx.payload.cpf_cnpj.clone());
        let company_name = (!is_cpf).then(|| ctx.payload.company_name.clone());
        let state_registration = (!is_cpf).then(|| ctx.payload.state_registration.clone());

        let new_customer = Customer {
            first_name: ctx.payload.first_name.clone(),
            last_name: ctx.payload.last_name.clone(),
            email: ctx.payload.email.clone(),
            password_hash: password::hash_password(&helpers::generate_random_token(32))?,
            phone: ctx.payload.phone.clone(),
            cpf: cpf,
            cnpj: cnpj,
            company_name: company_name,
            state_registration: state_registration,
            ..Default::default()
        };

        // Attempt to execute the creation in a single database transaction
        let transaction = async {
            let mut tx = pool.begin().await?;

            // 1. Create and retrieve the newly created customer
            let customer = Customer::create_tx(&new_customer, &mut tx).await?;

            // 2. Parse the address number and state, then build the address object
            let number: Option<i32> = ctx.payload.number.parse().ok();
            let state: Option<BrazilianStates> = ctx.payload.state.parse().ok();
            let address = Address {
                customer_id: customer.id,
                street: ctx.payload.street.clone(),
                number: number,
                complement: ctx.payload.complement.clone(),
                neighborhood: ctx.payload.neighborhood.clone(),
                city: ctx.payload.city.clone(),
                state: state,
                postcode: ctx.payload.postcode.clone(),
                ..Default::default()
            };

            // 3. Create the address
            Address::create_tx(&address, &mut tx).await?;

            // Commit transaction
            tx.commit().await?;
            Ok(())
        }
        .await;

        match transaction {
            Ok(_) => {
                session.insert("customer_success_flash", "Customer added successfully.".to_string()).await?;
                return Ok(Redirect::to("/admin/add-customer").into_response());
            }
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        "customers_email_key" => {
                            ctx.errors.insert("email".to_string(), "This email is already registered.".to_string());
                        }
                        "customers_cpf_key" => {
                            ctx.errors.insert("cpf_cnpj".to_string(), "This CPF is already registered.".to_string());
                        }
                        "customers_cnpj_key" => {
                            ctx.errors.insert("cpf_cnpj".to_string(), "This CNPJ is already registered.".to_string());
                        }
                        _ => {
                            ctx.errors
                                .insert("internal_error".to_string(), "A value submitted already exists in the database.".to_string());
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("Customer creation failed: {:?}", e);
                ctx.errors.insert(
                    "internal_error".to_string(),
                    "An unexpected database error occurred. Please try again.".to_string(),
                );
            }
        }
    }

    let template = admin_add_customer_template(&ctx);
    let html = admin_layout("Add Customer", template, &ctx, None);

    Ok(html.render().into_response())
}

pub async fn admin_customer_details(
    Path(id): Path<i64>,
    Extension(admin): Extension<Admin>,
    Extension(csrf_token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let customer = Customer::get_by_id(&id, &pool).await?;
    ctx.customer = customer;

    let address = Address::get_by_customer_id(&id, &pool).await?;
    ctx.address = address;

    let page: i64 = params.get("page").and_then(|v| v.parse().ok()).map(|v: i64| v.clamp(1, 10_000)).unwrap_or(1);
    let limit: i64 = params.get("limit").and_then(|v| v.parse().ok()).map(|v: i64| v.clamp(10, 100)).unwrap_or(10);

    let order_history = Order::get_paginated_by_customer_id(&ctx.customer.as_ref().unwrap().id, &page, &limit, "", &pool).await?;
    ctx.data = order_history;

    let template = admin_customer_details_template(&ctx);
    let html = admin_layout("Customer Details", template, &ctx, None);

    Ok(html.render().into_response())
}
