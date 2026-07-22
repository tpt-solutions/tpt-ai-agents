use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, PatType};

use crate::schema::function_to_json_schema;

/// Convert a `snake_case` identifier into `PascalCase` (e.g. `get_weather` -> `GetWeather`).
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Expand a `#[tool]` attribute into schema + wrapper code.
///
/// In addition to the JSON schema, this generates:
/// - An `Args` struct (`Serialize` + `Deserialize`) mirroring the function's parameters.
/// - A `<fn>_call` function that deserializes JSON tool-call arguments and invokes
///   the original function, so tool arguments never need manual (de)serialization.
pub fn expand_tool(input: &ItemFn) -> syn::Result<TokenStream> {
    let fn_name = &input.sig.ident;
    let schema = function_to_json_schema(input)?;

    let wrapper_name = format_ident!("{}_tool_schema", fn_name);
    let schema_fn_name = format_ident!("{}_schema", fn_name);
    let args_struct_name = format_ident!("{}Args", to_pascal_case(&fn_name.to_string()));
    let call_fn_name = format_ident!("{}_call", fn_name);

    let mut field_defs = Vec::new();
    let mut field_names = Vec::new();
    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let Pat::Ident(pat_ident) = pat.as_ref() {
                let ident = &pat_ident.ident;
                field_defs.push(quote! { pub #ident: #ty });
                field_names.push(ident.clone());
            }
        }
    }

    let output_ty = match &input.sig.output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, ty) => quote!(#ty),
    };

    let expanded = quote! {
        #input

        /// Auto-generated JSON schema for the tool.
        #[allow(non_upper_case_globals)]
        pub static #wrapper_name: &str = #schema;

        /// Get the JSON schema for this tool.
        pub fn #schema_fn_name() -> &'static str {
            #wrapper_name
        }

        /// Auto-generated argument struct for (de)serializing this tool's JSON arguments.
        #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
        pub struct #args_struct_name {
            #(#field_defs),*
        }

        /// Deserialize JSON tool-call arguments and invoke the underlying function.
        pub fn #call_fn_name(
            args_json: &str,
        ) -> ::core::result::Result<#output_ty, ::serde_json::Error> {
            let args: #args_struct_name = ::serde_json::from_str(args_json)?;
            Ok(#fn_name(#(args.#field_names),*))
        }
    };

    Ok(expanded)
}
