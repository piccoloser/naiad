use rocket::form::FromFormField;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Roles for users in the system. Defined in order of increasing privilege.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::User => write!(f, "user"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FormUserRole(pub UserRole);

impl<'v> FromFormField<'v> for FormUserRole {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        match UserRole::from_str(field.value) {
            Ok(role) => Ok(FormUserRole(role)),
            Err(_) => Err(rocket::form::Error::validation("invalid user role").into()),
        }
    }
}
