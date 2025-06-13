use super::error::INVALID_INPUT;
use crate::core::roles::{FormUserRole, UserRole};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, FromForm)]
pub struct AuthClaims {
    pub sub: String,
    pub exp: usize,
    #[field(name = "roles")]
    pub roles: Vec<FormUserRole>,
}

impl AuthClaims {
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&FormUserRole(UserRole::Admin))
    }

    pub fn can_access_user(&self, target_uid: &str) -> bool {
        self.is_admin() || self.sub == target_uid
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthClaims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(str::to_owned)
            .or_else(|| {
                req.cookies()
                    .get_private(crate::constants::TOKEN)
                    .map(|c| c.value().to_owned())
            });

        let token = match token {
            Some(t) => t,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let jwt_handler = match req
            .guard::<&rocket::State<std::sync::Arc<JwtHandler>>>()
            .await
        {
            Outcome::Success(state) => state,
            Outcome::Error(_) => return Outcome::Error((Status::InternalServerError, ())),
            Outcome::Forward(_) => return Outcome::Forward(Status::InternalServerError),
        };

        match jwt_handler.parse(&token) {
            Ok(claims) => Outcome::Success(claims),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct JwtHandler {
    secret_key: String,
}

impl JwtHandler {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub async fn check(&self, token: &str) -> bool {
        let token = match jsonwebtoken::decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &jsonwebtoken::Validation::new(Algorithm::HS256),
        ) {
            Ok(token) => token,
            Err(_) => return false,
        };

        token.claims.exp > Utc::now().timestamp() as usize
    }

    pub fn generate<T: AsRef<str>, U: AsRef<str>>(
        &self,
        identifier: T,
        expiration: U,
        roles: Vec<FormUserRole>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let exp_seconds: i64 = expiration.as_ref().parse().map_err(|_| INVALID_INPUT)?;
        let expiration = Utc::now() + Duration::seconds(exp_seconds);

        let claims = AuthClaims {
            sub: identifier.as_ref().to_string(),
            exp: expiration.timestamp() as usize,
            roles,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn parse(&self, token: &str) -> Result<AuthClaims, Status> {
        decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &jsonwebtoken::Validation::new(Algorithm::HS256),
        )
        .map(|token| token.claims)
        .map_err(|_| Status::Unauthorized)
    }
}
