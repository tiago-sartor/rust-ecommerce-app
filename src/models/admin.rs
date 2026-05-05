pub struct Admin {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: String,
    pub profile_image_url: Option<String>,
    pub role: AdminRole,
    pub is_active: bool,
    pub last_login: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub enum AdminRole {
    Admin,
    Manager,
    Editor,
    Support,
}

pub fn get_all_system_users() -> Vec<Admin> {
    // This function would typically fetch data from a database
    vec![
        Admin {
            id: 1,
            first_name: "Alice".to_string(),
            last_name: "Smith".to_string(),
            email: "alice.smith@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "123-456-7890".to_string(),
            role: AdminRole::Admin,
            is_active: true,
            last_login: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            profile_image_url: None,
        },
        Admin {
            id: 2,
            first_name: "Bob".to_string(),
            last_name: "Johnson".to_string(),
            email: "bob.johnson@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "9  87-654-3210".to_string(),
            role: AdminRole::Manager,
            is_active: true,
            last_login: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            profile_image_url: None,
        },
    ]
}
