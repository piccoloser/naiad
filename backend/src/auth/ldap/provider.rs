use super::{LdapConfig, LdapManager, LdapPool};
use crate::{
    auth::{error::MISSING_EMAIL_ATTRIBUTE, AuthError, Authenticator},
    core::roles::UserRole,
    entities::users::Model as User,
};
use deadpool::managed::Pool;
use ldap3::{LdapConnAsync, LdapConnSettings, SearchEntry};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct LdapProvider {
    pub config: Arc<LdapConfig>,
    pub pool: LdapPool,
}

impl LdapProvider {
    pub fn new(config: LdapConfig) -> Self {
        let arc_config = Arc::new(config);
        let manager = LdapManager::new(arc_config.clone());
        let pool = Pool::builder(manager).max_size(10).build().unwrap();

        Self {
            config: arc_config,
            pool,
        }
    }

    pub fn build_user_from_entry(search_entry: &SearchEntry) -> Result<User, AuthError> {
        let email = search_entry
            .attrs
            .get("mail")
            .and_then(|v| v.first())
            .ok_or(AuthError::LdapError(MISSING_EMAIL_ATTRIBUTE.to_string()))?;

        let username = search_entry
            .attrs
            .get("sAMAccountName")
            .and_then(|v| v.first())
            .map(|s| s.to_string());

        Ok(Self::generate_ldap_user(email, username.as_ref()))
    }

    pub fn extract_sam_account(email: &str) -> Result<&str, AuthError> {
        email.split('@').next().ok_or(AuthError::InvalidInput)
    }

    fn generate_ldap_user<T: AsRef<str>>(email: T, username: Option<T>) -> User {
        User {
            id: 0,
            email: email.as_ref().to_string(),
            username: username.map(|v| v.as_ref().to_string()),
            pw_hash: String::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            removed_at: None,
        }
    }

    async fn prepare_ldap_query(
        &self,
        identifier: &str,
    ) -> Result<(String, deadpool::managed::Object<LdapManager>, String), AuthError> {
        let sam_account = Self::extract_sam_account(identifier)?.to_string();
        let (bind_username, bind_password) = self.config.env_svc_account();
        let base_dn = self.config.build_base_dn();
        let search_filter = format!("(&(objectCategory=person)(sAMAccountName={sam_account}))");
        let mut ldap_conn = self
            .pool
            .get()
            .await
            .map_err(|e| AuthError::LdapError(e.to_string()))?;

        ldap_conn
            .simple_bind(
                &format!("{bind_username}@{}", self.config.domain),
                &bind_password,
            )
            .await
            .map_err(|e| AuthError::LdapError(e.to_string()))?
            .success()
            .map_err(|_| AuthError::InvalidCredentials)?;

        Ok((base_dn, ldap_conn, search_filter))
    }

    pub fn translate_group_to_role(group: &str) -> Option<UserRole> {
        match group.to_lowercase().as_str() {
            "label maker admin" => Some(UserRole::Admin),
            "label maker user" => Some(UserRole::User),
            _ => None,
        }
    }
}

impl Authenticator for LdapProvider {
    async fn fetch_roles(&self, user: &User) -> Result<Vec<UserRole>, AuthError> {
        let (base_dn, mut ldap_conn, search_filter) = self
            .prepare_ldap_query(&user.email)
            .await
            .map_err(|e| AuthError::AuthProviderError(e.to_string()))?;

        let (result, _) = ldap_conn
            .search(
                &base_dn,
                ldap3::Scope::Subtree,
                &search_filter,
                vec!["memberOf"],
            )
            .await
            .map_err(|e| AuthError::LdapError(e.to_string()))?
            .success()
            .map_err(|e| AuthError::LdapError(e.to_string()))?;

        let entry = result.first().ok_or(AuthError::InvalidCredentials)?;
        let search_entry = SearchEntry::construct(entry.clone());
        let roles = search_entry
            .attrs
            .get("memberOf")
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|dn| {
                dn.split(',')
                    .find(|s| s.trim_start().starts_with("CN="))
                    .map(|cn| cn.trim_start_matches("CN="))
                    .and_then(Self::translate_group_to_role)
            })
            .collect::<Vec<UserRole>>();

        Ok(roles)
    }

    async fn get_user_by_identifier(&self, identifier: &str) -> Result<User, AuthError> {
        let (base_dn, mut ldap_conn, search_filter) = self
            .prepare_ldap_query(identifier)
            .await
            .map_err(|e| AuthError::AuthProviderError(e.to_string()))?;

        let (result, _) = ldap_conn
            .search(
                &base_dn,
                ldap3::Scope::Subtree,
                &search_filter,
                vec!["dn", "mail", "sAMAccountName"],
            )
            .await
            .map_err(|_| AuthError::LdapSearchError)?
            .success()
            .map_err(|_| AuthError::LdapSearchError)?;

        let entry = result.first().ok_or(AuthError::InvalidCredentials)?;
        let search_entry = SearchEntry::construct(entry.clone());

        LdapProvider::build_user_from_entry(&search_entry)
    }

    async fn read_user(&self, email: &str, password: &str) -> Result<User, AuthError> {
        let (base_dn, mut ldap_conn, search_filter) = self
            .prepare_ldap_query(email)
            .await
            .map_err(|e| AuthError::AuthProviderError(e.to_string()))?;

        let (result, _) = ldap_conn
            .search(&base_dn, ldap3::Scope::Subtree, &search_filter, vec!["*"])
            .await
            .map_err(|e| AuthError::LdapError(e.to_string()))?
            .success()
            .map_err(|e| AuthError::LdapError(e.to_string()))?;

        let entry = result.first().ok_or(AuthError::InvalidCredentials)?;
        let search_entry = SearchEntry::construct(entry.clone());
        let user_dn = &search_entry.dn;
        let (tmp_conn, mut rebinding_ldap) = LdapConnAsync::with_settings(
            LdapConnSettings::new()
                .set_starttls(true)
                .set_no_tls_verify(true),
            &self.config.host,
        )
        .await
        .map_err(|e| AuthError::LdapError(e.to_string()))?;

        ldap3::drive!(tmp_conn);

        rebinding_ldap
            .simple_bind(user_dn, password)
            .await
            .map_err(|e| AuthError::LdapError(e.to_string()))?
            .success()
            .map_err(|_| AuthError::InvalidCredentials)?;

        LdapProvider::build_user_from_entry(&search_entry)
    }
}
