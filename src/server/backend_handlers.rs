use crate::backend::layouts::admin_layout::admin_layout;
use crate::backend::templates::admin_dashboard::admin_dashboard as admin_dashboard_template;
use crate::models::admin::{Admin, AdminRole};
use hypertext::Renderable;

pub async fn admin_dashboard() -> impl axum::response::IntoResponse {
    let admin = Admin {
        id: 1,
        first_name: "Tiago".to_string(),
        last_name: "Lino".to_string(),
        email: "tiago@example.com".to_string(),
        password_hash: "".to_string(),
        phone: "".to_string(),
        profile_image_url: None,
        role: AdminRole::Admin,
        is_active: true,
        last_login: None,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };

    admin_layout("Dashboard", admin_dashboard_template(&admin)).render()
}
