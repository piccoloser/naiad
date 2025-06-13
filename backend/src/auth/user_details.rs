use super::{AuthError, Authenticator};
use crate::core::{forms::LoginForm, roles::UserRole};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UserDetails {
    pub email: String,
    pub roles: Vec<UserRole>,
}

impl UserDetails {
    pub async fn build<A: Authenticator + Sync>(
        auth: &A,
        email: &str,
        password: &str,
    ) -> Result<Self, AuthError> {
        let user = auth
            .read_user(email, password)
            .await
            .map_err(|e| AuthError::AuthProviderError(e.to_string()))?;
        let email = user.email.clone();
        let roles = auth.fetch_roles(&user).await?;

        Ok(Self { email, roles })
    }

    pub async fn from_form<A: Authenticator + Sync>(
        auth: &A,
        form: &LoginForm,
    ) -> Result<Self, AuthError> {
        Self::build(auth, &form.email, &form.password).await
    }
}
