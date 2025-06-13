mod context;
mod docs;
mod utils;

use docs::generate_docs;
use utils::{
    generate_create_struct, generate_endpoint_and_handler_ident, generate_patch_logic,
    generate_patch_struct, generate_route_signature,
};

use heck::ToPascalCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

/// ## How to Use
/// ```rust
/// // Example for `users` table
/// #[generate_crud_routes(
///     crate::entities::users::Entity,
///     route = "users",
///     guards = { // Any or all of these guards can be omitted
///        create: crate::core::guards::SomeGuard,
///        read: crate::core::guards::SomeGuard,
///        update: crate::core::guards::SomeGuard,
///        delete: crate::core::guards::SomeGuard,
///     },
///     updatable = { // Fields not included here will not be updatable
///         username: Option<String>,
///         email: String,
///         pw_hash: String,
///     }
/// )]
/// async fn user_routes() {}
/// ```
///
/// This generates `PATCH`, `GET`, `POST`, and `DELETE` endpoints for a SeaORM Entity.
/// - `PATCH` updates only the provided fields (via JSON)
/// - `updated_at` is automatically refreshed
/// - Role guards are optional, via `create`/`read`/`update`/`delete` = `MyGuard`
#[proc_macro_attribute]
pub fn generate_crud_routes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let raw_args = parse_macro_input!(attr as context::CrudContext);
    let func = parse_macro_input!(item as ItemFn);
    let args = raw_args.finalize(func.sig.ident.clone());
    let endpoint = &args.endpoint();
    let fn_ident = &args.fn_ident();
    let base_ident = format_ident!("__{}_handler", fn_ident);
    let entity_ty = &args.generate_ty("Entity");
    let model_ty = &args.generate_ty("Model");
    let model_ident = &args.model_segment().ident;

    // Generate a struct to handle model creation
    let create_ident = format_ident!("__Create{}", &model_ident.to_string().to_pascal_case());
    let create_struct = generate_create_struct(&create_ident, &args.updatable);
    let field_names: Vec<_> = args.updatable.iter().map(|(ident, _)| ident).collect();

    // Generate a struct to handle model mutation
    let patch_ident = format_ident!("__Patch{}", &model_ident.to_string().to_pascal_case());
    let patch_struct = generate_patch_struct(&patch_ident, &args.updatable);
    let patch_logic = generate_patch_logic(entity_ty, &args.updatable, &args.timestamps);

    // Generate endpoints and handler identifiers
    let (a_endpoint, a_ident) = generate_endpoint_and_handler_ident(
        &args,
        "all",
        Some("all"),
        None,
        Some("<limit>&<offset>"),
    );

    let (o_endpoint, o_ident) =
        generate_endpoint_and_handler_ident(&args, "one", None, Some("<id>"), None);

    let (c_endpoint, c_ident) =
        generate_endpoint_and_handler_ident(&args, "create", Some("create"), None, None);

    let (u_endpoint, u_ident) =
        generate_endpoint_and_handler_ident(&args, "update", Some("update"), Some("<id>"), None);

    let (d_endpoint, d_ident) =
        generate_endpoint_and_handler_ident(&args, "delete", Some("delete"), Some("<id>"), None);

    // Generate handlers
    let c_handler = generate_route_signature(
        "post",
        &c_endpoint,
        &c_ident,
        args.guards.get("create"),
        quote! { input: rocket::serde::json::Json<#create_ident> },
        Some("<input>"),
    );

    let a_handler = generate_route_signature(
        "get",
        &a_endpoint,
        &a_ident,
        args.guards.get("read"),
        quote! { limit: Option<u64>, offset: Option<u64> },
        None,
    );

    let o_handler = generate_route_signature(
        "get",
        &o_endpoint,
        &o_ident,
        args.guards.get("read"),
        quote! { id: i32 },
        None,
    );

    let u_handler = generate_route_signature(
        "patch",
        &u_endpoint,
        &u_ident,
        args.guards.get("update"),
        quote! { id: i32, input: rocket::serde::json::Json<#patch_ident> },
        Some("<input>"),
    );

    let d_handler = generate_route_signature(
        "delete",
        &d_endpoint,
        &d_ident,
        args.guards.get("delete"),
        quote! { id: i32},
        None,
    );

    // Generate HTML documentation
    let route_docs = generate_docs(
        args.endpoint(),
        &[
            (&a_endpoint, "List all models"),
            (&o_endpoint, "Get a single model by ID"),
            (&c_endpoint, "Create a new model"),
            (&u_endpoint, "Update an existing model by ID"),
            (&d_endpoint, "Delete a model by ID"),
        ],
    );

    // Export generated code (structs, documentation, routes)
    let generated = quote! {
        #create_struct
        #patch_struct

        #[rocket::get(#endpoint)]
        async fn #base_ident(guard : Option<crate::core::guards::AdminGuard>) -> Result<rocket::response::content::RawHtml<&'static str>, rocket::response::Redirect> {
            #[cfg(debug_assertions)]
            {
                // In debug mode, just return the documentation immediately.
                return Ok(rocket::response::content::RawHtml(#route_docs));
            }

            #[cfg(not(debug_assertions))]
            {
                match guard {
                    Some(_) => Ok(rocket::response::content::RawHtml(#route_docs)),
                    None => Err(rocket::response::Redirect::to(uri!(#a_ident(
                        limit = Option::<u64>::None,
                        offset = Option::<u64>::None
                    )))),
                }
            }
        }

        // CREATE route
        #c_handler -> Result<rocket::serde::json::Json<#model_ty>, rocket::http::Status> {
            use sea_orm::{EntityTrait, ActiveModelTrait, IntoActiveModel};

            let db = db.inner().as_ref();
            let data = input.into_inner();
            let mut active: <#entity_ty as sea_orm::EntityTrait>::ActiveModel = std::default::Default::default();
            #(
                active.#field_names = sea_orm::Set(data.#field_names.clone());
            )*


            match <#entity_ty as EntityTrait>::insert(active).exec_with_returning(db).await {
                Ok(created) => Ok(rocket::serde::json::Json(created)),
                Err(_) => Err(rocket::http::Status::InternalServerError),
            }
        }

        // READ route (all)
        #a_handler -> Result<
            rocket::serde::json::Json<Vec<#model_ty>>,
            rocket::http::Status
        > {
            use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect};

            let limit = limit.unwrap_or(100);
            let offset = offset.unwrap_or(0);
            let db = db.inner().as_ref();

            let paginator = <#entity_ty as EntityTrait>::find()
                .limit(limit)
                .offset(offset)
                .paginate(db, limit);

            let results = paginator
                .fetch()
                .await
                .map_err(|_| rocket::http::Status::InternalServerError)?;

            Ok(rocket::serde::json::Json(results))
        }

        // READ route (one)
        #o_handler -> Result<rocket::serde::json::Json<#model_ty>, rocket::http::Status> {
            use sea_orm::{EntityTrait, QueryFilter};

            let db = db.inner().as_ref();

            match <#entity_ty as EntityTrait>::find_by_id(id)
                .one(db)
                .await {
                    Ok(Some(model)) => Ok(rocket::serde::json::Json(model)),
                    Ok(None) => Err(rocket::http::Status::NotFound),
                    Err(_) => Err(rocket::http::Status::InternalServerError),
                }
        }

        // PATCH route (update)
        #u_handler -> Result<rocket::serde::json::Json<#model_ty>, rocket::http::Status> {
            #patch_logic
        }

        // DELETE route
        #d_handler -> Result<rocket::http::Status, rocket::http::Status> {
            use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, IntoActiveModel};

            let db = db.inner().as_ref();

            let model = <#entity_ty as EntityTrait>::find_by_id(id)
                .one(db)
                .await
                .map_err(|_| rocket::http::Status::InternalServerError)?;

            let Some(model) = model else {
                return Err(rocket::http::Status::NotFound);
            };

            let active = model.into_active_model();

            active
                .delete(db)
                .await
                .map_err(|_| rocket::http::Status::InternalServerError)?;

            Ok(rocket::http::Status::NoContent)
        }

        // Create a function which returns the generated routes
        pub fn #fn_ident() -> Vec<rocket::Route> {
            rocket::routes![
                #base_ident,
                #a_ident,
                #o_ident,
                #c_ident,
                #u_ident,
                #d_ident,
            ]
        }
    };

    generated.into()
}
