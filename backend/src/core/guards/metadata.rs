use super::AdminGuard;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

pub struct PublicMetadata;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PublicMetadata {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-App-Internal") == Some("frontend") {
            true => Outcome::Success(PublicMetadata),
            false => Outcome::Error((Status::Forbidden, ())),
        }
    }
}

pub struct PrivateMetadata;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PrivateMetadata {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if req.headers().get_one("X-App-Internal") != Some("frontend") {
            return Outcome::Error((Status::Forbidden, ()));
        }

        match AdminGuard::from_request(req).await {
            Outcome::Success(_) => Outcome::Success(PrivateMetadata {}),
            Outcome::Error(e) => Outcome::Error(e),
            Outcome::Forward(_) => Outcome::Forward(Status::new(599)),
        }
    }
}
