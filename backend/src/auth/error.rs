use rocket::{http::Status, request::Request, response::Responder, serde::json::Json};

pub const ACCOUNT_ALREADY_EXISTS: &str = "Account already exists";
pub const INVALID_CREDENTIALS: &str = "Invalid credentials";
pub const INVALID_INPUT: &str = "Invalid input";
pub const LDAP_SEARCH_FAILED: &str = "LDAP search failed";
pub const MISSING_EMAIL_ATTRIBUTE: &str = "Missing email attribute";
pub const REGISTRATION_NOT_SUPPORTED: &str = "Registration not supported";
pub const RESOURCE_NOT_FOUND: &str = "Resource not found";
pub const ROLE_ASSIGNMENT_NOT_SUPPORTED: &str = "Role assignment not supported";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Debug)]
pub enum AuthError {
    AccountAlreadyExists,
    InvalidCredentials,
    InvalidInput,
    LdapSearchError,
    ResourceNotFound,
    AuthProviderError(String),
    DbError(String),
    DbQueryError(String),
    EncryptionError(String),
    JwtCreationError(String),
    LdapError(String),
    RegistrationError(String),
    SessionStoreError(String),
}

impl AuthError {
    fn http_status_and_msg(&self) -> (Status, String) {
        match self {
            Self::AccountAlreadyExists => (Status::Conflict, ACCOUNT_ALREADY_EXISTS.into()),
            Self::InvalidCredentials => (Status::Unauthorized, INVALID_CREDENTIALS.into()),
            Self::InvalidInput => (Status::BadRequest, INVALID_INPUT.into()),
            Self::LdapSearchError => (Status::InternalServerError, LDAP_SEARCH_FAILED.into()),
            Self::ResourceNotFound => (Status::NotFound, RESOURCE_NOT_FOUND.into()),
            Self::AuthProviderError(e) => (
                Status::InternalServerError,
                format!("AuthProviderError: {e:?}"),
            ),
            Self::DbError(e) => (Status::InternalServerError, format!("DbError: {e:?}")),
            Self::DbQueryError(e) => (Status::InternalServerError, format!("DbQueryError: {e:?}")),
            Self::EncryptionError(e) => (
                Status::InternalServerError,
                format!("EncryptionError: {e:?}"),
            ),
            Self::LdapError(e) => (Status::InternalServerError, format!("LdapError: {e:?}")),
            Self::JwtCreationError(e) => (
                Status::InternalServerError,
                format!("JwtCreationError: {e:?}"),
            ),
            Self::RegistrationError(e) => (Status::BadRequest, format!("RegistrationError: {e:?}")),
            Self::SessionStoreError(e) => (
                Status::InternalServerError,
                format!("SessionStoreError: {e:?}"),
            ),
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_, message) = self.http_status_and_msg();
        write!(f, "{message}")
    }
}

impl std::error::Error for AuthError {}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AuthError {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, message) = self.http_status_and_msg();
        Json(ErrorMessage { message })
            .respond_to(_req)
            .map(|mut r| {
                r.set_status(status);
                r
            })
    }
}
