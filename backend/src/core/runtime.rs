use super::server::ServerHandler;
use rocket::{Build, Rocket};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc;

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub async fn run_server<F, Fut>(build_rocket: F) -> Result<(), rocket::Error>
where
    F: FnOnce(Arc<ServerHandler>) -> Fut + Clone + Send + 'static,
    Fut: std::future::Future<Output = Rocket<Build>> + Send + 'static,
{
    loop {
        let (restart_tx, mut restart_rx) = mpsc::channel::<()>(1);
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        let server_handler = Arc::new(ServerHandler::new(restart_tx, shutdown_tx));
        let rocket = (build_rocket.clone())(server_handler.clone());
        let rocket_handle = tokio::spawn(rocket.await.launch());

        tokio::select! {
            _ = restart_rx.recv() => {
                tracing::info!("Restarting server...");
                rocket_handle.abort();
                continue;
            },
            _ = shutdown_rx.recv() => {
                tracing::info!("Server shutdown signal received.");
                SHUTDOWN.store(true, Ordering::SeqCst);
                rocket_handle.abort();
                return Ok(());
            },
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("\nReceived SIGINT. Shutting down server...");
                SHUTDOWN.store(true, Ordering::SeqCst);
                rocket_handle.abort();
                return Ok(());
            }
        }
    }
}
