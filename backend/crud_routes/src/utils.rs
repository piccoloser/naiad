use crate::context::CrudContext;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Path, Type};

/// Returns a tuple consisting of:
/// 1. The full endpoint string for use in `rocket::get`, `rocket::post`, etc.
/// 2. The identifier for the handler function (for internal use).
///
/// **Parameters:**
/// 1. `endpoint`: The base endpoint string (e.g., `/users`).
/// 2. `ident`: The identifier of the function being decorated.
/// 3. `append`: A string to append to the endpoint (e.g., `all`, `one`, etc.).
/// 4. `path_params`: Optional string for path parameters (e.g., `<id>`).
/// 5. `query_params`: Optional string for query parameters (e.g., `<limit>&<offset>`).
pub fn generate_endpoint_and_handler_ident(
    args: &CrudContext,
    ident_alias: &str,
    slug: Option<&str>,
    path_params: Option<&str>,
    query_params: Option<&str>,
) -> (String, Ident) {
    let mut segments = vec![args.endpoint()];

    if let Some(s) = slug {
        if !s.is_empty() {
            segments.push(s);
        }
    }

    if let Some(p) = path_params {
        segments.push(p);
    }

    let joined = segments.join("/");

    let full = if let Some(q) = query_params {
        format!("{joined}?{q}")
    } else {
        joined
    };

    let handler_ident = format_ident!("__{}_{ident_alias}", args.fn_ident());

    (full, handler_ident)
}

/// Generates a struct used to deserialize POST input during model creation.
///
/// Fields are taken directly from the macro's `updatable = {}` list,
/// and will be required exactly as declared (no `Option<T>` wrapping).
pub fn generate_create_struct(create_ident: &Ident, fields: &[(Ident, Type)]) -> TokenStream2 {
    let field_defs = fields.iter().map(|(ident, ty)| {
        quote! { pub #ident: #ty, }
    });

    quote! {
        #[derive(Debug, serde::Deserialize)]
        struct #create_ident {
            #(#field_defs)*
        }
    }
}

/// Generates a struct used to deserialize PATCH input during partial updates.
///
/// Fields are automatically wrapped in `Option<T>` so clients can selectively
/// update fields. Unknown fields will trigger an error due to `deny_unknown_fields`.
pub fn generate_patch_struct(patch_ident: &Ident, fields: &[(Ident, Type)]) -> TokenStream2 {
    let field_defs = fields.iter().map(|(ident, ty)| {
        quote! { pub #ident: Option<#ty>, }
    });

    quote! {
        #[derive(Clone, Debug, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct #patch_ident {
            #(#field_defs)*
        }
    }
}

/// Generates the logic for PATCH requests in the `/update` route handler.
///
/// This performs a conditional field-by-field update on the provided ActiveModel
/// base on which fields were present in the JSON input.
///
/// If `timestamps` is true, the `updated_at` field is automatically refreshed.
pub fn generate_patch_logic(
    entity_ty: &Path,
    fields: &[(Ident, Type)],
    timestamps: &bool,
) -> TokenStream2 {
    let mut patch_statements: Vec<TokenStream2> = fields
        .iter()
        .map(|(ident, _ty)| {
            quote! {
                if let Some(v) = input.#ident.as_ref() {
                    active.#ident = sea_orm::Set(v.clone().into());
                }
            }
        })
        .collect();

    if *timestamps {
        patch_statements.push(quote! {
            active.updated_at = sea_orm::Set(::chrono::Utc::now());
        });
    }

    quote! {
        use sea_orm::{EntityTrait, ActiveModelTrait, IntoActiveModel, QuerySelect, Set};

        let db = db.inner().as_ref();

        let Some(model) = <#entity_ty as EntityTrait>::find_by_id(id)
            .one(db)
            .await
            .map_err(|_| rocket::http::Status::InternalServerError)? else {
                return Err(rocket::http::Status::NotFound);
            };

        let mut active = model.into_active_model();

        #(#patch_statements)*

        let updated = active.update(db)
            .await
            .map_err(|_| rocket::http::Status::InternalServerError)?;

        Ok(rocket::serde::json::Json(updated))
    }
}

/// Generates a Rocket route signature based on:
/// - HTTP method (e.g., `get`, `post`)
/// - The endpoint string (e.g., `/users/<id>`)
/// - An optional guard (e.g., `AtLeastUser`, `AtLeastAdmin`)
/// - Extra arguments (e.g., `input`)
/// - Optional `data = "<input>"` annotations
pub fn generate_route_signature(
    method: &str,
    endpoint: &str,
    ident: &Ident,
    guard: Option<&syn::Path>,
    args: TokenStream2,
    data_param: Option<&str>,
) -> TokenStream2 {
    let method = format_ident!("{method}");
    let data_attr = if let Some(data_param) = data_param {
        quote! { , data = #data_param }
    } else {
        quote! {}
    };

    if let Some(guard) = guard {
        quote! {
            #[rocket::#method(#endpoint #data_attr)]
            async fn #ident(
                _guard: #guard,
                db: &rocket::State<std::sync::Arc<sea_orm::DatabaseConnection>>,
                #args
            )
        }
    } else {
        quote! {
            #[rocket::#method(#endpoint #data_attr)]
            async fn #ident(
                db: &rocket::State<std::sync::Arc<sea_orm::DatabaseConnection>>,
                #args
            )
        }
    }
}
