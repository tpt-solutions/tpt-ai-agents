use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, PatType, Type};

/// Generate a JSON schema string from a function signature.
pub fn function_to_json_schema(input: &ItemFn) -> syn::Result<String> {
    let mut properties = Vec::new();
    let mut required = Vec::new();

    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            let name = extract_param_name(pat);
            let is_optional = is_option_type(ty);
            let json_type = rust_type_to_json(ty)?;
            properties.push(format!("\"{name}\": {{\"type\": \"{json_type}\"}}"));
            if !is_optional {
                required.push(name);
            }
        }
    }

    let properties_str = properties.join(",");

    let required_str = required
        .iter()
        .map(|r| format!("\"{r}\""))
        .collect::<Vec<_>>()
        .join(", ");

    let description = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                attr.meta.require_name_value().ok().and_then(|nv| {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = &nv.value
                    {
                        Some(s.value().trim().to_string())
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    Ok(format!(
        "{{\"type\":\"object\",\"properties\":{{{}}},\"required\":[{}],\"description\":\"{}\"}}",
        properties_str, required_str, description
    ))
}

/// Extract the parameter's identifier (e.g. `location` from `location: String`).
fn extract_param_name(pat: &syn::Pat) -> String {
    match pat {
        syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
        _ => String::from("param"),
    }
}

fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|s| s.ident == "Option")
            .unwrap_or(false),
        _ => false,
    }
}

fn rust_type_to_json(ty: &Type) -> syn::Result<&'static str> {
    match ty {
        Type::Path(type_path) => {
            let name = type_path
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            match name.as_str() {
                "String" | "str" => Ok("string"),
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => Ok("integer"),
                "f32" | "f64" => Ok("number"),
                "bool" => Ok("boolean"),
                "Vec" | "VecDeque" => Ok("array"),
                "Option" => Ok("string"),
                _ => Ok("string"),
            }
        }
        _ => Ok("string"),
    }
}

/// Expand a `#[derive(JsonSchema)]` into schema generation.
pub fn expand_json_schema(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let name_str = name.to_string();

    let expanded = quote! {
        impl #name {
            /// Get the JSON schema for this type.
            pub fn json_schema() -> &'static str {
                concat!("{\"type\":\"object\",\"description\":\"", #name_str, "\"}")
            }
        }
    };

    Ok(expanded)
}
