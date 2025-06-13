#[macro_use]
extern crate rocket;

#[macro_use]
extern crate crud_routes;

use config_tools::{sectioned_defaults, Config, LoadOutcome, Section};
use rocket::fs::FileServer;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::sync::{Mutex as AsyncMutex, RwLock};
use tracing_subscriber::EnvFilter;

mod auth;
mod constants;
mod core;
mod entities;

use auth::{DbProvider, JwtHandler, LdapConfig, LdapProvider, SessionManager};
use constants::{
    ADDRESS, APP, CONFIG_INI, DATABASE, DATABASE_URL, DEFAULT_HOST,
    DEFAULT_JWT_EXP_SECONDS, DEFAULT_PORT, DOMAIN, ENV_DATABASE_URL,HOST,
    JWT_EXP_SECONDS, LDAP, LDAP_DOMAIN, LDAP_HOST, PORT, SERVER, SUPPORT_EMAIL,
};
use sea_orm::{Database, DatabaseConnection};

use crate::constants::DIST_DIRECTORY;

// The title of the application. Adjust as needed.
pub const APP_TITLE: &str = "Naiad Application";

pub type SharedAuthProvider = Arc<RwLock<auth::AuthProvider>>;
pub type SharedConfig = Arc<AsyncMutex<Config>>;

pub fn opt_env_var(key: &str) -> String {
    match env::var(key.to_uppercase()) {
        Ok(value) => value,
        Err(e) => {
            tracing::info!("Optional environment variable {key:?}: {e:?}");
            "".to_string()
        }
    }
}

async fn get_auth_provider(config: &Config, db: &Arc<DatabaseConnection>) -> auth::AuthProvider {
    if let Some(ldap_section) = config.section(LDAP) {
        if let Ok(ldap_config) = LdapConfig::from_section(ldap_section) {
            if ldap_config.is_configured() {
                return auth::AuthProvider::Ldap(LdapProvider::new(ldap_config));
            }
        }
    }

    auth::AuthProvider::Database(DbProvider::new(db))
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();
    
    let config = match config_tools::Config::load_or_default_outcome(
        CONFIG_INI,
        sectioned_defaults! {
            [APP] {
                SUPPORT_EMAIL => "",
                JWT_EXP_SECONDS => DEFAULT_JWT_EXP_SECONDS,
            }

            [DATABASE] {
                DATABASE_URL => &ENV_DATABASE_URL.to_string(),
            }

            [SERVER] {
                ADDRESS => DEFAULT_HOST,
                PORT => DEFAULT_PORT,
            }

            [LDAP] {
                HOST => &opt_env_var(LDAP_HOST),
                DOMAIN => &opt_env_var(LDAP_DOMAIN),
            }
        },
    ) {
        LoadOutcome::FromFile(config) => config,
        LoadOutcome::FromDefault(config) => config.save(CONFIG_INI).unwrap().clone(),
    };

    let db = Arc::new(
        Database::connect(
            config
                .get(Some(DATABASE), DATABASE_URL)
                .expect("Missing database URL configuration value"),
        )
        .await
        .expect("Failed to connect to the database"),
    );

    let secret_key = Arc::new(core::generate_secret_key());

    let jwt_handler = JwtHandler::new(secret_key.to_string());
    let request_client = reqwest::Client::new();
    let session_manager = SessionManager::new(&db);

    core::runtime::run_server(move |server_handler| {
        let config = Config::load(CONFIG_INI).expect("Failed to load configuration file");

        /// Helper function to reduce boilerplate in this closure.
        fn share<T: Clone + Send + Sync + 'static>(item: T) -> Arc<T> {
            Arc::new(item.clone())
        }

        let db = db.clone();
        let jwt_handler = share(jwt_handler);
        let session_manager = share(session_manager);
        let request_client = share(request_client);
        let secret_key = secret_key.clone();
        let config = Arc::new(AsyncMutex::new(config));

        async move {
            let (auth_provider, support_email) = {
                let cfg = config.lock().await;
                (
                    Arc::new(RwLock::new(get_auth_provider(&cfg, &db).await)),
                    cfg.get(Some(APP), SUPPORT_EMAIL)
                        .expect("Missing support email configuration value")
                        .to_string(),
                )
            };

            let state = Arc::new(Mutex::new(
                core::State::new(APP_TITLE)
                    .with_data(SUPPORT_EMAIL, support_email.to_string()),
            ));

            let rocket = core::lifecycle::prepare_rocket(
                secret_key.to_string(),
                &config.clone(),
                server_handler.clone(),
            )
            .await
            .expect("Failed to prepare Rocket instance");

            rocket
                .manage(auth_provider)
                .manage(config)
                .manage(jwt_handler)
                .manage(db)
                .manage(session_manager)
                .manage(request_client)
                .manage(state)
                .mount("/", FileServer::from(&*DIST_DIRECTORY).rank(1))
                .mount("/", routes![core::api::spa_fallback])
                .mount("/auth", core::api::auth::routes())
                .mount("/app", core::api::metadata::routes())
                .mount("/entities", core::api::entity::routes())
                .mount("/server", core::server::routes())
        }
    })
    .await
}
