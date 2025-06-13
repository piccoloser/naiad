use crate::auth::AuthClaims;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct ClaimsGuard(pub AuthClaims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClaimsGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.guard::<AuthClaims>().await {
            Outcome::Success(claims) => Outcome::Success(ClaimsGuard(claims)),
            Outcome::Forward(status) => Outcome::Forward(status),
            Outcome::Error((status, _)) => Outcome::Error((status, ())),
        }
    }
}
