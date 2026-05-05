use super::address::Address;

pub struct Customer {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: String,
    pub profile_image_url: Option<String>,
    pub cpf: Option<String>,
    pub company_name: Option<String>,
    pub cnpj: Option<String>,
    pub state_registration: Option<String>,
    pub address: Option<Address>,
    pub is_active: bool,
    pub last_login: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_customer_by_id(customer_id: u32) -> Option<Customer> {
    // Simulate fetching customer from a database
    Some(Customer {
        id: customer_id,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        phone: "1234567890".to_string(),
        profile_image_url: None,
        cpf: None,
        company_name: None,
        cnpj: None,
        state_registration: None,
        address: None,
        is_active: true,
        last_login: None,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    })
}

pub async fn get_customer_by_email(email: &str) -> Option<Customer> {
    // Simulate fetching customer from a database
    if email == "john.doe@example.com" {
        Some(Customer {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: email.to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "1234567890".to_string(),
            profile_image_url: None,
            cpf: None,
            company_name: None,
            cnpj: None,
            state_registration: None,
            address: None,
            is_active: true,
            last_login: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
    } else {
        None
    }
}

pub async fn get_customer_by_cpf(cpf: &str) -> Option<Customer> {
    // Simulate fetching customer from a database
    if cpf == "123.456.789-00" {
        Some(Customer {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "1234567890".to_string(),
            profile_image_url: None,
            cpf: Some("123.456.789-00".to_string()),
            company_name: None,
            cnpj: None,
            state_registration: None,
            address: None,
            is_active: true,
            last_login: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
    } else {
        None
    }
}

pub async fn get_customer_by_cnpj(cnpj: &str) -> Option<Customer> {
    // Simulate fetching customer from a database
    if cnpj == "12.345.678/0001-00" {
        Some(Customer {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            phone: "1234567890".to_string(),
            profile_image_url: None,
            cpf: None,
            company_name: Some("Awesome Company LLC".to_string()),
            cnpj: Some("12.345.678/0001-00".to_string()),
            state_registration: None,
            address: None,
            is_active: true,
            last_login: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
    } else {
        None
    }
}

pub async fn create_customer(customer: Customer) -> Customer {
    // Simulate creating a customer in a database
    customer
}

pub async fn update_customer(customer_id: u32, updated_customer: Customer) -> Option<Customer> {
    // Simulate updating a customer in a database
    if customer_id == 1 {
        Some(updated_customer)
    } else {
        None
    }
}

pub async fn delete_customer(customer_id: u32) -> bool {
    // Simulate deleting a customer from a database
    customer_id == 1
}
