pub fn routes() -> Vec<rocket::Route> {
    vec![
        crate::core::api::auth::routes(),
        user_routes(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[generate_crud_routes(
    module = crate::entities::users,
    route = "users",
    guards = {
        read: crate::core::guards::SameUserOrAdmin,
        update: crate::core::guards::SameUserOrAdmin,
        delete: crate::core::guards::AtLeastAdmin,
    },
    updatable = {
        email: String,
        username: Option<String>,
        pw_hash: String,
    }
)]
pub async fn user_routes() {}
