use super::error::{REGISTRATION_NOT_SUPPORTED, ROLE_ASSIGNMENT_NOT_SUPPORTED};
use super::{AuthError, Authenticator, DbProvider, LdapProvider, UserCreator};
use crate::constants::{DATABASE, LDAP};
use crate::core::{forms::RegisterForm, roles::UserRole};
use crate::entities::users::Model as User;

#[derive(Clone, Debug)]
pub enum AuthProvider {
    Database(DbProvider),
    Ldap(LdapProvider),
}

impl AuthProvider {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Database(_) => DATABASE,
            Self::Ldap(_) => LDAP,
        }
    }

    /// **Developers:**
    ///
    /// Define registration variants both here and in `impl UserCreator for AuthProvider`.
    pub fn supports_registration(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Self::Database(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for AuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(_) => write!(f, "Database"),
            Self::Ldap(_) => write!(f, "LDAP"),
        }
    }
}

impl Authenticator for AuthProvider {
    async fn fetch_roles(&self, user: &User) -> Result<Vec<UserRole>, AuthError> {
        match self {
            Self::Database(p) => p.fetch_roles(user).await,
            Self::Ldap(p) => p.fetch_roles(user).await,
        }
    }

    async fn get_user_by_identifier(&self, identifier: &str) -> Result<User, AuthError> {
        match self {
            Self::Database(p) => p.get_user_by_identifier(identifier).await,
            Self::Ldap(p) => p.get_user_by_identifier(identifier).await,
        }
    }

    async fn read_user(&self, email: &str, password: &str) -> Result<User, AuthError> {
        match self {
            Self::Database(p) => p.read_user(email, password).await,
            Self::Ldap(p) => p.read_user(email, password).await,
        }
    }
}

impl UserCreator for AuthProvider {
    async fn assign_role(&self, user: User, role: UserRole) -> Result<(), AuthError> {
        match self {
            Self::Database(p) => p.assign_role(user, role).await,
            _ => Err(AuthError::AuthProviderError(
                ROLE_ASSIGNMENT_NOT_SUPPORTED.to_string(),
            )),
        }
    }

    async fn create_user(&self, form: RegisterForm) -> Result<User, AuthError> {
        match self {
            Self::Database(p) => p.create_user(form).await,
            _ => Err(AuthError::AuthProviderError(
                REGISTRATION_NOT_SUPPORTED.to_string(),
            )),
        }
    }
}
