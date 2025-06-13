use crate::constants::CONFIG_INI;
use crate::core::guards::{PrivateMetadata, PublicMetadata};
use crate::core::{response::AuthInfoResponse, State as AppState};
use crate::{get_auth_provider, SharedAuthProvider, SharedConfig};
use config_tools::Config;
use rocket::{http::Status, serde::json::Json, State};
use sea_orm::DatabaseConnection;
use std::sync::{Arc, Mutex};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        auth_info,
        get_config,
        get_state,
        set_config,
        update_auth_provider,
    ]
}

#[get("/auth_info")]
pub async fn auth_info(auth: &State<SharedAuthProvider>) -> Json<AuthInfoResponse> {
    let auth = auth.as_ref().read().await;
    Json(AuthInfoResponse {
        kind: auth.kind(),
        supports_registration: auth.supports_registration(),
    })
}

#[get("/config")]
pub async fn get_config(
    _auth: PrivateMetadata,
    state: &State<SharedConfig>,
) -> Result<Json<Config>, Status> {
    let config = state.lock().await.clone();
    Ok(Json(config))
}

#[post("/config", data = "<input>")]
pub async fn set_config(
    _auth: PrivateMetadata,
    config: &State<SharedConfig>,
    input: Json<Config>,
) -> Result<Status, Status> {
    let input = input.into_inner();
    let mut config = config.lock().await;

    for (k, v) in &input.general_values {
        config.update(None, k, v);
    }

    for (section, values) in &input.sections {
        for (k, v) in values {
            config.update(Some(section), k, v);
        }
    }

    if let Err(e) = config.save(CONFIG_INI) {
        tracing::error!("Failed to save config: {e}");
        return Err(Status::InternalServerError);
    }

    Ok(Status::NoContent)
}

#[get("/state")]
pub async fn get_state(
    _auth: PublicMetadata,
    state: &State<Arc<Mutex<AppState>>>,
) -> Result<Json<crate::core::State>, Status> {
    let state = state
        .lock()
        .map_err(|e| {
            tracing::error!("Failed to lock state: {e}");
            Status::InternalServerError
        })?
        .clone();
    Ok(Json(state))
}

#[post("/update_auth_provider")]
pub async fn update_auth_provider(
    _auth: PrivateMetadata,
    config: &State<SharedConfig>,
    db: &State<Arc<DatabaseConnection>>,
    provider: &State<SharedAuthProvider>,
) -> Result<Status, Status> {
    let cfg = config.lock().await;
    let new_provider = get_auth_provider(&cfg, db).await;
    *provider.write().await = new_provider;
    tracing::info!("Auth provider updated successfully");
    Ok(Status::NoContent)
}
