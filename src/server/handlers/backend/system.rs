use crate::backend::{layouts::*, templates::*};
use crate::emails::mailer::{EmailLog, Mailer, Status};
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::utils::context::Context;
use crate::utils::errors::AppError;
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
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_dashboard_template(&ctx);
    Ok(admin_layout("Dashboard", template, &ctx).render().into_response())
}

pub async fn admin_account(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_account_template(&ctx);
    Ok(admin_layout("Account", template, &ctx).render().into_response())
}

pub async fn admin_settings(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_settings_template(&ctx);
    Ok(admin_layout("Settings", template, &ctx).render().into_response())
}

pub async fn admin_emails(
    State(pool): State<PgPool>,
    Extension(admin): Extension<Admin>,
    Extension(csrf_token): Extension<CsrfToken>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::<(), (Vec<EmailLog>, i64, i64, i64, Option<Status>)>::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let page: i64 = params.get("page").and_then(|v| v.parse().ok().filter(|&v| v > 0)).unwrap_or(1);
    let limit: i64 = params.get("limit").and_then(|v| v.parse().ok().filter(|&v| v > 9)).unwrap_or(10);
    let filter_by: Option<Status> = params.get("filter_by").and_then(|v| v.parse().ok());

    let (logs, count) = Mailer::get_logs_paginated(page, limit, &filter_by, &pool).await.unwrap_or_default();
    ctx.data = (logs, count, page, limit, filter_by);

    let template = admin_emails_template(&ctx);
    Ok(admin_layout("Email Logs", template, &ctx).render().into_response())
}

pub async fn admin_email_details(Path(id): Path<i64>, State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let details = Mailer::get_log_details(id, &pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(details))
}

pub async fn admin_email_resend(Path(id): Path<i64>, State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let mailer = Mailer::new(&pool).map_err(|e| AppError::Internal(e.to_string()))?;
    mailer.resend_email(id).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Redirect::to("/admin/emails"))
}

pub async fn admin_email_delete(Path(id): Path<i64>, State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    Mailer::delete_log(id, &pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Redirect::to("/admin/emails"))
}

pub async fn admin_email_bulk_actions(State(pool): State<PgPool>, Form(payload): Form<HashMap<String, String>>) -> Result<impl IntoResponse, AppError> {
    let action = payload.get("action").map(|s| s.as_str()).unwrap_or_default();
    let ids_json = payload.get("ids").map(|s| s.as_str()).unwrap_or("[]");
    let ids: Vec<i64> = serde_json::from_str(ids_json).unwrap_or_default();

    if !ids.is_empty() {
        match action {
            "resend" => {
                let mailer = Mailer::new(&pool).map_err(|e| AppError::Internal(e.to_string()))?;
                mailer.bulk_resend(&ids).await.map_err(|e| AppError::Internal(e.to_string()))?;
            }
            "delete" => {
                Mailer::bulk_delete(&ids, &pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
            }
            _ => {}
        }
    }

    Ok(Redirect::to("/admin/emails"))
}
