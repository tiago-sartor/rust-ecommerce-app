//! Request payloads and validation structures for backend handlers.
//!
//! This module defines the request bodies and query parameters expected by
//! various backend endpoints, using the `validator` crate for input validation
//! and `serde` for serialization/deserialization.

use serde::{Deserialize, Deserializer, Serialize};
use validator::Validate;

use crate::utils::cpf_cnpj::{sanitize_input, validate_cpf_cnpj};

// =========================================================================
// Authentication Payloads
// =========================================================================

/// Request payload for logging in a backend user (e.g., administrator).
#[derive(Deserialize, Validate, Default)]
pub struct LoginPayload {
    /// The user's email address, validated to ensure correct format.
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,

    /// The user's password.
    pub password: String,
}

/// Request payload for initiating a password reset request.
#[derive(Deserialize, Validate, Default)]
pub struct ForgotPasswordPayload {
    /// The email address associated with the account requesting the reset.
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
}

/// Request payload for completing a password reset using a reset token.
#[derive(Deserialize, Validate, Default)]
pub struct ResetPasswordPayload {
    /// The new password (must be between 8 and 32 characters long).
    #[validate(length(min = 8, max = 32, message = "Password must be between 8 and 32 characters"))]
    pub password: String,

    /// Confirmation of the new password, which must match the `password` field.
    #[validate(must_match(other = "password", message = "Password does not match"))]
    pub confirm_password: String,
}

// =========================================================================
// Customer Management Payloads
// =========================================================================

/// Request payload for creating or adding a new customer, including
/// personal identification, contact information, and shipping/billing address.
#[derive(Deserialize, Serialize, Validate, Default)]
pub struct AddCustomerPayload {
    /// The customer's first name (minimum 3 characters).
    #[validate(length(min = 3, max = 100, message = "First name must be at least 3 characters"))]
    pub first_name: String,

    /// The customer's last name (minimum 3 characters).
    #[validate(length(min = 3, max = 100, message = "Last name must be at least 3 characters"))]
    pub last_name: String,

    /// The customer's CPF or CNPJ identification number.
    /// Sanitized during deserialization and validated using a custom validator.
    #[serde(deserialize_with = "sanitize_cpf_cnpj")]
    #[validate(custom(function = "validate_cpf_cnpj"))]
    pub cpf_cnpj: String,

    /// The customer's state registration ID (optional, maximum 100 characters).
    #[validate(length(max = 100, message = "State registration is too long"))]
    pub state_registration: String,

    /// The customer's company name (optional, maximum 100 characters).
    #[validate(length(max = 100, message = "Company name is too long"))]
    pub company_name: String,

    /// The customer's email address, validated to ensure correct format.
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,

    /// The customer's phone number (must be between 10 and 11 digits).
    #[serde(deserialize_with = "sanitize_digits")]
    #[validate(length(min = 10, max = 11, message = "Phone must be between 10 and 11 digits"))]
    pub phone: String,

    /// The postcode (CEP) of the customer's address (must be exactly 8 digits).
    /// Sanitized during deserialization to remove non-digit characters.
    #[serde(deserialize_with = "sanitize_digits")]
    #[validate(length(equal = 8, message = "Postcode must be exactly 8 digits"))]
    pub postcode: String,

    /// The street name of the customer's address.
    #[validate(length(min = 3, max = 100, message = "Street must be between 3 and 100 characters"))]
    pub street: String,

    /// The street number of the customer's address.
    #[serde(deserialize_with = "sanitize_digits")]
    #[validate(length(max = 6, message = "Please enter a valid number"))]
    pub number: String,

    /// Additional address details such as apartment or building number.
    #[validate(length(max = 100, message = "Complement is too long"))]
    pub complement: String,

    /// The neighborhood or district of the customer's address.
    #[validate(length(min = 3, max = 100, message = "Neighborhood must be between 3 and 100 characters"))]
    pub neighborhood: String,

    /// The city of the customer's address.
    #[validate(length(min = 3, max = 100, message = "City must be between 3 and 100 characters"))]
    pub city: String,

    /// The 2-letter state acronym (e.g., SP, RJ).
    #[validate(length(equal = 2, message = "State must be a 2-letter acronym"))]
    pub state: String,
}

// =========================================================================
// Deserialization Helpers
// =========================================================================

/// Remove non-alphanumeric characters and sanitize a CPF/CNPJ input.
fn sanitize_cpf_cnpj<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|s| sanitize_input(&s))
}

/// Sanitizes user input by removing non-digit characters.
fn sanitize_digits<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|s| s.chars().filter(|c| c.is_ascii_digit()).collect())
}
