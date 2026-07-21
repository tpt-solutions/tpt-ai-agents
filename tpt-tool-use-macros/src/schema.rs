use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, PatType, Type};

/// Generate a JSON schema string from a function signature.
pub fn function_to_json_schema(input: &ItemFn) -> syn::Result<String> {
    let mut properties = String::new();
    let mut required = Vec::new();

    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            let (name, is_optional) = extract_param_info(ty)?;
            let json_type = rust_type_to_json(ty)?;
            properties.push_str(&format!(
                "\"{}\": {{\"type\": \"{}\"}},",
                name, json_type
            ));
            if !is_optional {
                required.push(name);
            }
        }
    }

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
                attr.meta
                    .require_name_value()
                    .ok()
                    .and_then(|nv| {
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
        properties, required_str, description
    ))
}

fn extract_param_info(ty: &Type) -> syn::Result<(String, bool)> {
    match ty {
        Type::Path(type_path) => {
            let name = type_path
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            let is_optional = name == "Option";
            Ok((name.to_lowercase(), is_optional))
        }
        _ => Ok(("param".into(), false)),
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
