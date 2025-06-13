mod db;
mod enums;
mod error;
mod jwt;
mod ldap;
mod session;
mod user_details;

use crate::core::{forms::RegisterForm, roles::UserRole};
use crate::entities::users::Model as User;
pub use db::DbProvider;
pub use enums::AuthProvider;
pub use error::AuthError;
pub use jwt::{AuthClaims, JwtHandler};
pub use ldap::{LdapConfig, LdapProvider};
pub use session::SessionManager;
pub use user_details::UserDetails;

pub trait Authenticator {
    async fn fetch_roles(&self, user: &User) -> Result<Vec<UserRole>, AuthError>;
    async fn get_user_by_identifier(&self, identifier: &str) -> Result<User, AuthError>;
    async fn read_user(&self, email: &str, password: &str) -> Result<User, AuthError>;
}

pub trait UserCreator {
    async fn assign_role(&self, user: User, role: UserRole) -> Result<(), AuthError>;
    async fn create_user(&self, form: RegisterForm) -> Result<User, AuthError>;
}
