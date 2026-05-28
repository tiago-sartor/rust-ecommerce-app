use crate::backend::{layouts::*, templates::*};
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::utils::context::Context;
use crate::utils::errors::AppError;
use axum::{extract::Extension, response::IntoResponse};
use hypertext::Renderable;

pub async fn admin_orders(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_orders_template(&ctx);
    Ok(admin_layout("Orders", template, &ctx).render().into_response())
}

pub async fn admin_add_order_get(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_add_order_template(&ctx);
    Ok(admin_layout("Add Order", template, &ctx).render().into_response())
}

pub async fn admin_order_details(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_order_details_template(&ctx);
    Ok(admin_layout("Order Details", template, &ctx).render().into_response())
}

pub async fn admin_customers(Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.csrf_token = csrf_token;

    let template = admin_add_customer_template(&ctx);
    Ok(admin_layout("Customers", template, &ctx).render().into_response())
}

pub async fn admin_add_customer_get(Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.csrf_token = csrf_token;

    let template = admin_add_customer_template(&ctx);
    Ok(admin_layout("Customers", template, &ctx).render().into_response())
}
