# tpt-tool-use-macros

[![crates.io](https://img.shields.io/crates/v/tpt-tool-use-macros.svg)](https://crates.io/crates/tpt-tool-use-macros)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Proc macros exposing Rust functions as LLM tools.

## Usage

```rust,ignore
use tpt_tool_use_macros::tool;

/// Get the current weather for a location.
#[tool]
fn get_weather(location: String, units: Option<String>) -> String {
    format!("Weather for {} at 72°F", location)
}

// Generates: get_weather_schema() -> &'static str (JSON schema)
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
