use crate::core::guards::ClaimsGuard;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct SameUserOrAdmin;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SameUserOrAdmin {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let claims = match req.guard::<ClaimsGuard>().await {
            Outcome::Success(claims) => claims.0,
            Outcome::Forward(status) => return Outcome::Forward(status),
            Outcome::Error((status, _)) => return Outcome::Error((status, ())),
        };

        let route_id = match req.param::<String>(1) {
            Some(id) => id,
            _ => return Outcome::Error((Status::BadRequest, ())),
        }
        .unwrap();

        if claims.can_access_user(&route_id) {
            Outcome::Success(SameUserOrAdmin)
        } else {
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}
