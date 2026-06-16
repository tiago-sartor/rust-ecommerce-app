use crate::backend::{layouts::*, templates::*};
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::utils::{AppError, Context};
use axum::{extract::Extension, response::IntoResponse};
use hypertext::Renderable;

pub async fn admin_products(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_products_template(&ctx);
    let html = admin_layout("Products", template, &ctx, None);
    Ok(html.render().into_response())
}

pub async fn admin_add_product(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_add_product_template(&ctx);
    let html = admin_layout("Add Product", template, &ctx, None);
    Ok(html.render().into_response())
}

pub async fn admin_edit_product(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_edit_product_template(&ctx);
    let html = admin_layout("Edit Product", template, &ctx, None);
    Ok(html.render().into_response())
}
