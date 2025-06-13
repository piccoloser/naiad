mod claims;
mod metadata;
mod role;
mod same_or_admin;

pub use claims::ClaimsGuard;
pub use metadata::{PrivateMetadata, PublicMetadata};
pub use role::RoleGuard;
pub use same_or_admin::SameUserOrAdmin;

use crate::core::roles::UserRole;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[macro_export]
macro_rules! define_role_guard {
    ($name:ident, $role:expr) => {
        #[allow(dead_code)]
        pub struct $name(pub $crate::core::guards::RoleGuard);

        impl $name {
            #[allow(dead_code)]
            pub fn inner(&self) -> $crate::auth::UserDetails {
                self.0.inner()
            }
        }

        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for $name {
            type Error = ();
            async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                match $crate::core::guards::RoleGuard::from_request(req).await {
                    Outcome::Success(guard) => {
                        if guard.has_role($role) {
                            Outcome::Success($name(guard))
                        } else {
                            Outcome::Error((Status::Forbidden, ()))
                        }
                    }
                    Outcome::Error(e) => Outcome::Error(e),
                    Outcome::Forward(_) => Outcome::Forward(Status::new(599)),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_role_threshold_guard {
    ($name:ident, $role:expr) => {
        #[allow(dead_code)]
        pub struct $name(pub $crate::core::guards::RoleGuard);

        impl $name {
            #[allow(dead_code)]
            pub fn inner(&self) -> $crate::auth::UserDetails {
                self.0.inner()
            }
        }

        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for $name {
            type Error = ();
            async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                match $crate::core::guards::RoleGuard::from_request(req).await {
                    Outcome::Success(guard) => {
                        if guard.has_at_least($role) {
                            Outcome::Success($name(guard))
                        } else {
                            Outcome::Error((Status::Forbidden, ()))
                        }
                    }
                    Outcome::Error(e) => Outcome::Error(e),
                    Outcome::Forward(_) => Outcome::Forward(Status::new(599)),
                }
            }
        }
    };
}

// Define guards for each role
define_role_guard!(AdminGuard, UserRole::Admin);
define_role_guard!(UserGuard, UserRole::User);

// Define threshold guards for minimum role permissions
define_role_threshold_guard!(AtLeastAdmin, UserRole::Admin);
define_role_threshold_guard!(AtLeastUser, UserRole::User);
