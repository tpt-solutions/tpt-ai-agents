# tpt-prompt-template

[![crates.io](https://img.shields.io/crates/v/tpt-prompt-template.svg)](https://crates.io/crates/tpt-prompt-template)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Safe, typed templating for prompts with compile-time validation.

## Features

- `std` (default): Enables standard library features
- Compile-time validation of `{{variable}}` syntax

## Usage

```rust,ignore
use tpt_prompt_template::{PromptTemplate, template};

let tmpl = PromptTemplate::new("Hello {{name}}, topic: {{topic}}").unwrap();
let rendered = tmpl.render(&[("name", "Alice"), ("topic", "Rust")]);
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
