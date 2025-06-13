use crate::core::guards::AtLeastAdmin;
use config_tools::FromSection;
use rocket::{serde::json::Json, State};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

/// A simple message wrapper returned from control endpoints like `/restart` and `/shutdown`.
///
/// Useful for sending simple status messages in JSON responses.
#[derive(serde::Serialize)]
pub struct Message {
    pub message: String,
}

impl Message {
    pub fn new<T: AsRef<str>>(message: T) -> Self
    where
        std::string::String: From<T>,
    {
        Self {
            message: message.into(),
        }
    }
}

/// Represents the server's address and port configuration, loaded from
/// the `[server]` section of the configuration file.
#[derive(Debug, FromSection)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

/// Internal controller for triggering server lifecycle events like restart and shutdown.
pub struct ServerHandler {
    restart_tx: Sender<()>,
    shutdown_tx: Sender<()>,
}

impl ServerHandler {
    pub fn new(restart_tx: Sender<()>, shutdown_tx: Sender<()>) -> Self {
        Self {
            restart_tx,
            shutdown_tx,
        }
    }
}

/// - `/restart` - Triggers a hot-reload of the server.
/// - `/shutdown` - Triggers a graceful shutdown of the server.
pub fn routes() -> Vec<rocket::Route> {
    routes![restart, shutdown]
}

/// Triggers a hot-reload of the server.
#[get("/restart")]
pub async fn restart(
    _admin: AtLeastAdmin,
    server_handler: &State<Arc<ServerHandler>>,
) -> Json<Message> {
    match server_handler.as_ref().restart_tx.send(()).await {
        Ok(_) => Json(Message::new("Restarting server...")),
        Err(e) => {
            tracing::error!("Failed to send restart signal: {}", e);
            std::process::exit(1)
        }
    }
}

/// Triggers a graceful shutdown of the server.
#[get("/shutdown")]
pub async fn shutdown(
    _admin: AtLeastAdmin,
    server_handler: &State<Arc<ServerHandler>>,
) -> Json<Message> {
    match server_handler.as_ref().shutdown_tx.send(()).await {
        Ok(_) => Json(Message::new("Shutting down server...")),
        Err(e) => {
            tracing::error!("Failed to send shutdown signal: {}", e);
            std::process::exit(1)
        }
    }
}
