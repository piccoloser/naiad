use once_cell::sync::Lazy;
use std::{env, path::PathBuf};

pub static DIST_DIRECTORY: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(rocket::fs::relative!("../frontend/dist/client")));

pub static ENV_DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db".to_string()));

// Client messages
pub const EMAIL_REQUIRED: &str = "Email is required";
pub const PASSWORDS_DO_NOT_MATCH: &str = "Passwords do not match";

// Default values
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "8000";
pub const DEFAULT_JWT_EXP_SECONDS: &str = "86400"; // 24 hours

// Important constants
pub const SECRET_KEY_LENGTH: usize = 32;

// Named constants
pub const ADDRESS: &str = "address";
pub const APP: &str = "app";
pub const CONFIG_INI: &str = "config.ini";
pub const DATABASE: &str = "database";
pub const DATABASE_URL: &str = "database_url";
pub const DOMAIN: &str = "domain";
pub const HOST: &str = "host";
pub const JWT_EXP_SECONDS: &str = "jwt_exp_seconds";
pub const LDAP: &str = "ldap";
pub const LDAP_DOMAIN: &str = "ldap_domain";
pub const LDAP_HOST: &str = "ldap_host";
pub const PORT: &str = "port";
pub const SECRET_KEY: &str = "secret_key";
pub const SERVER: &str = "server";
pub const SUPPORT_EMAIL: &str = "support_email";
pub const TOKEN: &str = "token";
