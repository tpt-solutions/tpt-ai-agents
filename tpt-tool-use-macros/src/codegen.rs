use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemFn;

use crate::schema::function_to_json_schema;

/// Expand a `#[tool]` attribute into schema + wrapper code.
pub fn expand_tool(input: &ItemFn) -> syn::Result<TokenStream> {
    let fn_name = &input.sig.ident;
    let schema = function_to_json_schema(input)?;

    let wrapper_name = format_ident!("{}_tool_schema", fn_name);
    let schema_fn_name = format_ident!("{}_schema", fn_name);

    let expanded = quote! {
        #input

        /// Auto-generated JSON schema for the tool.
        pub static #wrapper_name: &str = #schema;

        /// Get the JSON schema for this tool.
        pub fn #schema_fn_name() -> &'static str {
            #wrapper_name
        }
    };

    Ok(expanded)
}
