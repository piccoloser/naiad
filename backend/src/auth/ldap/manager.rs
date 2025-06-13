use super::config::LdapConfig;
use deadpool::managed::{Manager, RecycleError};
use ldap3::{Ldap, LdapConnAsync, LdapConnSettings};
use std::{future::Future, sync::Arc};

#[derive(Debug)]
pub struct LdapManager {
    config: Arc<LdapConfig>,
}

impl LdapManager {
    pub fn new(config: Arc<LdapConfig>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Manager for LdapManager {
    type Type = Ldap;
    type Error = ldap3::LdapError;

    fn create(
        &self,
    ) -> impl Future<Output = Result<<Self as Manager>::Type, <Self as Manager>::Error>> + Send
    {
        let config = self.config.clone();

        Box::pin(async move {
            let host = &config.host;
            let (ldap_conn, ldap) = LdapConnAsync::with_settings(
                LdapConnSettings::new()
                    .set_starttls(true)
                    .set_no_tls_verify(true),
                host,
            )
            .await?;

            ldap3::drive!(ldap_conn);
            Ok(ldap)
        })
    }

    fn recycle(
        &self,
        _conn: &mut Self::Type,
        _metrics: &deadpool::managed::Metrics,
    ) -> impl Future<Output = Result<(), RecycleError<<Self as Manager>::Error>>> + Send {
        Box::pin(async move {
            // Optionally add logging or verification logic here using `conn` and `metrics`
            Ok(())
        })
    }
}
