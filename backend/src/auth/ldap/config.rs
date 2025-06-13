use config_tools::FromSection;
use std::env;

#[derive(Debug, serde::Deserialize, serde::Serialize, FromSection)]
pub struct LdapConfig {
    pub host: String,
    pub domain: String,
}

impl LdapConfig {
    pub fn build_base_dn(&self) -> String {
        self.domain
            .split('.')
            .map(|part| format!("dc={}", part))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn is_configured(&self) -> bool {
        !self.host.is_empty() && !self.domain.is_empty()
    }

    pub fn env_svc_account(&self) -> (String, String) {
        const LDAP_BIND_USERNAME: &str = "ldap_bind_username";
        const LDAP_BIND_PASSWORD: &str = "ldap_bind_password";

        (
            env::var(LDAP_BIND_USERNAME).unwrap_or_default(),
            env::var(LDAP_BIND_PASSWORD).unwrap_or_default(),
        )
    }
}
