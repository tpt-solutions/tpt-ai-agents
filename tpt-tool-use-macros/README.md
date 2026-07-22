# tpt-tool-use-macros

[![crates.io](https://img.shields.io/crates/v/tpt-tool-use-macros.svg)](https://crates.io/crates/tpt-tool-use-macros)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Proc macros exposing Rust functions as LLM tools.

**When to use this crate:** you want to give a model a callable tool
without hand-writing its JSON schema or the argument-parsing boilerplate —
`#[tool]` generates both from the function signature.

## Usage

```rust,ignore
use tpt_tool_use_macros::tool;

/// Get the current weather for a location.
#[tool]
fn get_weather(location: String, units: Option<String>) -> String {
    format!("Weather for {} at 72°F", location)
}

// Also generates:
// - `GetWeatherArgs`, a `Serialize + Deserialize` struct mirroring the parameters
// - `get_weather_call(args_json: &str) -> Result<String, serde_json::Error>`,
//   which deserializes a tool call's JSON arguments and invokes `get_weather`
```

See [`examples/full_agent_loop.rs`](../examples/full_agent_loop.rs) in the
workspace root for a tool actually being called in a mock conversation.

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
