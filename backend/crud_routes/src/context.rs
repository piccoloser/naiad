use std::collections::HashMap;
use syn::{
    Ident, LitBool, LitStr, Path, Token, Type, braced,
    parse::{Parse, ParseStream},
};

/// Stores all arguments and derived values used by the `#[generate_crud_routes]` macro.
///
/// Raw values like `route`, `guards`, and `updatable` are parsed from the attribute,
/// while derived values like `endpoint` and `fn_ident` are set later by calling `finalize()`
pub struct CrudContext {
    pub entity_path: Path,
    pub route: Option<LitStr>,
    pub guards: HashMap<String, Path>,
    pub updatable: Vec<(Ident, Type)>,
    pub timestamps: bool,

    // Derived fields
    pub endpoint: Option<String>,
    pub fn_ident: Option<Ident>,
}

impl CrudContext {
    /// Completes the setup for `CrudContext` by injecting the function identifier
    /// and computing a default route path if none was provided.
    ///
    /// Must be called before using `endpoint()` or `fn_ident()`.
    pub fn finalize(mut self, fn_ident: Ident) -> Self {
        use heck::ToSnakeCase;

        self.fn_ident = Some(fn_ident);

        let snake_case = self
            .entity_path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .to_snake_case();

        let route = self
            .route
            .as_ref()
            .map(|s| s.value())
            .unwrap_or_else(|| snake_case);

        self.endpoint = Some(format!("/{}", route));

        self
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint
            .as_deref()
            .expect("Endpoint not set. Did you call finalize()?")
    }

    /// Returns the base module path without the `Entity` segment.
    ///
    /// Used for constructing other type paths like `Model`, `ActiveModel`, etc.
    pub fn base_path(&self) -> Path {
        let mut segments = self.entity_path.segments.clone();
        segments.pop(); // Remove the last segment (the model name)
        Path {
            leading_colon: self.entity_path.leading_colon,
            segments,
        }
    }

    pub fn fn_ident(&self) -> &Ident {
        self.fn_ident
            .as_ref()
            .expect("Function identifier not set. Did you call finalize()?")
    }

    /// Appends the given type suffix to the base path, creating a `Path` like `crate::entities::my_model::Entity`.
    ///
    /// Used to dynamically reference SeaORM types like `Entity`, `Model`, `ActiveModel`, etc.
    pub fn generate_ty(&self, suffix: &str) -> Path {
        let mut base = self.base_path();
        base.segments.push(syn::PathSegment {
            ident: Ident::new(suffix, proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        base
    }

    /// Returns the path segment immediately before `Entity` (usually the model name).
    /// e.g. `my_model` in `crate::entities::my_model::Entity`.
    ///
    /// Used for naming patch/create structs and other derived identifiers.
    pub fn model_segment(&self) -> &syn::PathSegment {
        self.entity_path
            .segments
            .iter()
            .rev()
            .nth(1)
            .expect("Module path should have at least two segment (e.g. crate::entities::my_model)")
    }
}

impl Parse for CrudContext {
    /// Parses macro input into a `CrudContext`.
    ///
    /// Accepts:
    /// - `module = crate::entities::my_model`
    /// - `route = "custom-route"`
    /// - `guards = { create: GuardA, read: GuardB, ... }`
    /// - `updatable = { field1: Type1, field2: Type2, ... }`
    /// - `timestamps = true | false`
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut module_path: Option<Path> = None;
        let mut route = None;
        let mut guards = HashMap::new();
        let mut updatable = Vec::new();
        let mut timestamps = true;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "module" => {
                    module_path = Some(input.parse()?);
                }
                "route" => {
                    route = Some(input.parse()?);
                }
                "guards" => {
                    let content;
                    braced!(content in input);
                    while !content.is_empty() {
                        let guard: Ident = content.parse()?;
                        content.parse::<Token![:]>()?;
                        let path: Path = content.parse()?;
                        guards.insert(guard.to_string(), path);
                        let _ = content.parse::<Token![,]>().ok();
                    }
                }
                "updatable" => {
                    let content;
                    braced!(content in input);
                    while !content.is_empty() {
                        let field: Ident = content.parse()?;
                        content.parse::<Token![:]>()?;
                        let ty: Type = content.parse()?;
                        updatable.push((field, ty));
                        let _ = content.parse::<Token![,]>().ok();
                    }
                }
                "timestamps" => {
                    let v: LitBool = input.parse()?;
                    timestamps = v.value;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        key,
                        format!("Unexpected key: {other}"),
                    ));
                }
            }

            let _ = input.parse::<Token![,]>().ok();
        }

        let module_path =
            module_path.ok_or_else(|| syn::Error::new(input.span(), "Module path is required"))?;

        let mut entity_path = module_path.clone();
        entity_path.segments.push(syn::PathSegment {
            ident: Ident::new("Entity", proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::None,
        });

        Ok(Self {
            entity_path,
            route,
            guards,
            updatable,
            timestamps,
            endpoint: None,
            fn_ident: None,
        })
    }
}
