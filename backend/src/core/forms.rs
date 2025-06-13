use crate::constants::{EMAIL_REQUIRED, PASSWORDS_DO_NOT_MATCH};
use rocket::form::FromForm;

#[derive(FromForm)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
#[derive(FromForm)]
pub struct RegisterForm {
    pub username: Option<String>,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl RegisterForm {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() {
            return Err(EMAIL_REQUIRED.into());
        }

        if self.password != self.confirm_password {
            return Err(PASSWORDS_DO_NOT_MATCH.into());
        }

        // TODO: Check password strength

        Ok(())
    }
}
