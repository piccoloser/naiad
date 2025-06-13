use crate::auth::UserDetails;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthInfoResponse {
    pub kind: &'static str,
    pub supports_registration: bool,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub logged_in: bool,
    pub message: String,
    pub user: Option<UserDetails>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}
