//! Proc macros exposing Rust functions as LLM tools.
//!
//! Generates JSON schemas from Rust function signatures and handles
//! automatic serialization/deserialization of tool arguments.
//!
//! # Example
//!
//! ```rust,ignore
//! use tpt_tool_use_macros::tool;
//!
//! /// Get the current weather for a location.
//! #[tool]
//! fn get_weather(location: String, units: Option<String>) -> String {
//!     format!("Weather for {} at 72°F", location)
//! }
//!
//! // The macro generates, alongside the original function:
//! // - `get_weather_schema() -> &'static str`, a JSON schema for the function
//! // - `GetWeatherArgs`, a `Serialize + Deserialize` struct mirroring the parameters
//! // - `get_weather_call(args_json: &str) -> Result<String, serde_json::Error>`,
//! //   which deserializes JSON tool-call arguments and invokes `get_weather`
//! //
//! // Requires `serde` and `serde_json` as dependencies of the crate using `#[tool]`.
//! ```
extern crate proc_macro;

mod codegen;
mod schema;

use proc_macro::TokenStream;

/// Derive macro to expose a Rust function as an LLM tool.
///
/// Generates a JSON schema from the function signature and creates
/// wrapper types for automatic argument serialization.
#[proc_macro_attribute]
pub fn tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    let input = match syn::parse::<syn::ItemFn>(item) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    match codegen::expand_tool(&input) {
        Ok(expanded) => expanded.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive macro to generate a JSON schema from a struct.
#[proc_macro_derive(JsonSchema)]
pub fn derive_json_schema(input: TokenStream) -> TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    match schema::expand_json_schema(&input) {
        Ok(expanded) => expanded.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
