use crate::auth::{JwtHandler, SessionManager, UserDetails};
use crate::{constants::TOKEN, core::roles::UserRole};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};
use std::sync::Arc;

pub struct RoleGuard {
    pub user: UserDetails,
}

#[allow(dead_code)]
impl RoleGuard {
    pub fn has_role(&self, role: UserRole) -> bool {
        self.user.roles.contains(&role)
    }

    pub fn has_at_least(&self, minimum: UserRole) -> bool {
        self.user.roles.iter().any(|role| role >= &minimum)
    }

    pub fn inner(&self) -> UserDetails {
        self.user.clone()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RoleGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jwt = match req.guard::<&State<Arc<JwtHandler>>>().await {
            Outcome::Success(jwt) => jwt,
            _ => return Outcome::Error((Status::Unauthorized, ())),
        };
        let ses = match req.guard::<&State<Arc<SessionManager>>>().await {
            Outcome::Success(ses) => ses,
            _ => return Outcome::Error((Status::Unauthorized, ())),
        };
        let token = match req.cookies().get_private(TOKEN) {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let token_value = token.value();

        if !ses.validate(jwt, token_value).await.unwrap_or(false) {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let claims = match jwt.parse(token_value) {
            Ok(claims) => claims,
            Err(_) => return Outcome::Error((Status::Unauthorized, ())),
        };

        let roles: Vec<UserRole> = claims.roles.into_iter().map(|r| r.0).collect();
        let user = UserDetails {
            email: claims.sub,
            roles,
        };

        Outcome::Success(RoleGuard { user })
    }
}
