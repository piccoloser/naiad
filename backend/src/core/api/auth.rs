use crate::auth::{AuthError, JwtHandler, UserCreator, UserDetails};
use crate::constants::{APP, DEFAULT_JWT_EXP_SECONDS, JWT_EXP_SECONDS, TOKEN};
use crate::core::{
    forms::{LoginForm, RegisterForm},
    guards::{AtLeastAdmin, AtLeastUser},
    roles::{FormUserRole, UserRole},
};
use crate::{SessionManager, SharedAuthProvider};
use config_tools::Config;
use rocket::{
    form::Form,
    http::{Cookie, CookieJar, SameSite},
    response::Redirect,
    serde::json::Json,
    State,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn routes() -> Vec<rocket::Route> {
    routes![clear_sessions, login, logout, me, register]
}

#[post("/clear_sessions")]
pub async fn clear_sessions(
    _auth: AtLeastAdmin,
    session_manager: &State<Arc<SessionManager>>,
) -> Result<Redirect, AuthError> {
    match session_manager.clear().await {
        Ok(_) => Ok(Redirect::to("/")),
        Err(e) => {
            tracing::error!("Failed to clear sessions: {e:?}");
            Err(AuthError::SessionStoreError(e.to_string()))
        }
    }
}

#[post("/login", data = "<form>")]
pub async fn login(
    jar: &CookieJar<'_>,
    auth: &State<SharedAuthProvider>,
    config: &State<Arc<Mutex<Config>>>,
    jwt_handler: &State<Arc<JwtHandler>>,
    session_manager: &State<Arc<SessionManager>>,
    form: Form<LoginForm>,
) -> Result<Redirect, AuthError> {
    let auth = auth.as_ref().read().await;
    let config = config.lock().await.clone();
    let form = form.into_inner();
    let user = UserDetails::from_form(&*auth, &form).await?;
    let token = jwt_handler
        .generate(
            &user.email,
            config
                .get(Some(APP), JWT_EXP_SECONDS)
                .unwrap_or(DEFAULT_JWT_EXP_SECONDS.to_string()),
            user.roles.into_iter().map(FormUserRole).collect(),
        )
        .map_err(|e| {
            tracing::error!("Failed to generate JWT: {e:?}");
            AuthError::JwtCreationError(e.to_string())
        })?;

    session_manager.add(&token).await?;
    let cookie = Cookie::build((TOKEN, token.clone()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict);
    jar.add_private(cookie);

    Ok(Redirect::to("/"))
}

#[get("/logout")]
pub async fn logout(
    jar: &CookieJar<'_>,
    session_manager: &State<Arc<SessionManager>>,
) -> Result<Redirect, AuthError> {
    let token = jar
        .get_private(TOKEN)
        .ok_or(AuthError::ResourceNotFound)?
        .value()
        .to_owned();

    match session_manager.invalidate(&token).await {
        Ok(_) => {
            jar.remove_private(Cookie::from(TOKEN));
            Ok(Redirect::to("/"))
        }
        Err(e) => {
            tracing::error!("Failed to invalidate session: {e:?}");
            Err(AuthError::ResourceNotFound)
        }
    }
}

#[get("/me")]
pub async fn me(user: AtLeastUser) -> Json<UserDetails> {
    Json(user.inner())
}

#[post("/register", data = "<form>")]
pub async fn register(
    auth: &State<SharedAuthProvider>,
    form: Form<RegisterForm>,
) -> Result<Redirect, AuthError> {
    let auth = auth.as_ref().read().await;
    let form = form.into_inner();
    if let Err(e) = form.validate() {
        return Err(AuthError::RegistrationError(e.to_string()));
    }

    match auth.create_user(form).await {
        Ok(user) => {
            auth.assign_role(user, UserRole::User).await.map_err(|e| {
                tracing::error!("Failed to assign default role: {e:?}");
                AuthError::AuthProviderError(e.to_string())
            })?;

            Ok(Redirect::to("/"))
        }
        Err(e) => Err(AuthError::AuthProviderError(e.to_string())),
    }
}
