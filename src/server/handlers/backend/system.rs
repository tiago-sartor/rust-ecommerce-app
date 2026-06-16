use crate::backend::{layouts::*, templates::*};
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::utils::{AppError, Context};
use axum::{
    Form,
    extract::{Extension, Path, Query, State},
    response::{IntoResponse, Json, Redirect},
};
use hypertext::Renderable;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn admin_dashboard(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_dashboard_template(&ctx);
    let html = admin_layout("Dashboard", template, &ctx, None);
    Ok(html.render().into_response())
}

pub async fn admin_account(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_account_template(&ctx);
    let html = admin_layout("Account", template, &ctx, None);
    Ok(html.render().into_response())
}

pub async fn admin_settings(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = Some(admin);
    ctx.csrf_token = csrf_token;

    let template = admin_settings_template(&ctx);
    let html = admin_layout("Settings", template, &ctx, None);
    Ok(html.render().into_response())
}
