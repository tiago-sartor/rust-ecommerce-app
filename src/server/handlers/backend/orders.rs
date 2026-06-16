use crate::backend::{layouts::*, templates::*};
use crate::middlewares::csrf::CsrfToken;
use crate::models::{Address, Admin, Customer};
use crate::server::handlers::backend::AddCustomerPayload;
use crate::utils::{AppError, BrazilianStates, Context};
use axum::{
    extract::{Extension, Form, State},
    response::{IntoResponse, Redirect},
};
use hypertext::Renderable;
use sqlx::PgPool;
use tower_sessions::Session;
use validator::Validate;

pub async fn admin_orders(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_orders_template(&ctx);

    let html = admin_layout("Orders", template, &ctx, None);
    Ok(html.render().into_response())
}

pub async fn admin_add_order_get(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_add_order_template(&ctx);

    let html = admin_layout("Add Order", template, &ctx, None);
    Ok(html.render().into_response())
}

pub async fn admin_order_details(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_order_details_template(&ctx);

    let html = admin_layout("Order Details", template, &ctx, None);
    Ok(html.render().into_response())
}
