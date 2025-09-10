use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Debug, Validate, Default, Serialize, Deserialize, Clone)]
pub struct LoginUserDTO {
    #[validate(length(min = 6, message = "Email must be at least 6 characters long"))]
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password must be at least 1 character long"))]
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Debug, Validate, Default, Serialize, Deserialize, Clone)]
pub struct RegisterUserDTO {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,
    #[validate(
        length(min = 6, message = "Email must be at least 6 characters long"),
        email(message = "Email must be a valid email address")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Password must be at least 1 character long"))]
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,

    #[validate(
        length(min = 1, message = "Password confirmation is required"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub password_confirm: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDTO {
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilterUserDTO {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDTO {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDTO {
            id: user.id.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            role: user.role.to_str().to_string(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub fn filter_users(user: &[User]) -> Vec<FilterUserDTO> {
        user.iter().map(FilterUserDTO::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDTO {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDTO {
    pub status: String,
    pub users: Vec<FilterUserDTO>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDTO {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewUpdateDTO {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RoleUpdateDto {
    #[validate(custom = "validate_user_role")]
    pub role: UserRole,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        _ => Err(validator::ValidationError::new("Invalid user role")),
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default)]
pub struct UpdatePasswordUpdateDto {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub old_password: String,
    #[validate(length(min = 1, message = "New password is required"))]
    #[validate(length(min = 6, message = "New password must be at least 6 characters long"))]
    pub new_password: String,
    #[validate(
        length(min = 1, message = "Password confirmation is required"),
        must_match(other = "new_password", message = "Passwords do not match")
    )]
    pub new_password_confirm: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default)]
pub struct ForgotPasswordRequestDTO {
    #[validate(length(min = 6, message = "Email must be at least 6 characters long"))]
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default)]
pub struct ResetPasswordRequestDTO {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
    #[validate(length(min = 1, message = "New password is required"))]
    #[validate(length(min = 6, message = "New password must be at least 6 characters long"))]
    pub new_password: String,
    #[validate(
        length(min = 1, message = "Password confirmation is required"),
        must_match(other = "new_password", message = "Passwords do not match")
    )]
    pub new_password_confirm: String,
}
