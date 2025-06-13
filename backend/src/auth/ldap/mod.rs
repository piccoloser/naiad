mod config;
mod manager;
mod provider;

pub use config::LdapConfig;
pub use manager::LdapManager;
pub use provider::LdapProvider;

pub type LdapPool = deadpool::managed::Pool<LdapManager>;
