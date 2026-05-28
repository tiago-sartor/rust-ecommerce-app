use crate::backend::{layouts::*, templates::*};
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::utils::errors::AppError;
use crate::utils::context::Context;
use axum::{extract::Extension, response::IntoResponse};
use hypertext::Renderable;

pub async fn admin_products(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_products_template(&ctx);
    Ok(admin_layout("Products", template, &ctx).render().into_response())
}

pub async fn admin_add_product(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_add_product_template(&ctx);
    Ok(admin_layout("Add Product", template, &ctx).render().into_response())
}

pub async fn admin_edit_product(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_edit_product_template(&ctx);
    Ok(admin_layout("Edit Product", template, &ctx).render().into_response())
}
