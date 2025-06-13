use crate::constants::{ADDRESS, PORT, SECRET_KEY, SERVER};

use super::server::{ServerConfig, ServerHandler};
use config_tools::{Config, Section};
use rocket::{Build, Rocket};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Builds a configured Rocket instance using values from the runtime configuration.
///
/// The Rocket server will use the provided `Config` to fill out the address and port
/// the server will listen on. It uses the provided secret key for signing cookies. The
/// server handler will be managed automatically. None of these values need to be
/// managed after passing them into this function; you can just `.manage()` state and
/// `.mount()` routes to the output directly.
///
/// ```rust
/// // Example
/// let rocket = core::ignite::prepare_rocket(
///     my_secret_key, &my_config, my_server_handler.clone()
/// ).await?;
///
/// rocket
///     .manage(state)
///     .mount("/", routes![])
///     // And so on...
/// ```
pub async fn prepare_rocket(
    secret_key: String,
    config: &Arc<Mutex<Config>>,
    server_handler: Arc<ServerHandler>,
) -> Result<Rocket<Build>, rocket::Error> {
    let config = config.lock().await.clone();

    let server_section = config
        .section(SERVER)
        .expect("Failed to get server section");

    let server_config =
        ServerConfig::from_section(server_section).expect("Failed to parse server config");

    let figment = rocket::Config::figment()
        .merge((ADDRESS, server_config.address))
        .merge((PORT, server_config.port))
        .merge((SECRET_KEY, &secret_key));

    Ok(rocket::custom(figment)
        .manage(server_handler)
        .manage(secret_key))
}
