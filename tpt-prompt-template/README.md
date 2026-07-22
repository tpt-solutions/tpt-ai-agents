# tpt-prompt-template

[![crates.io](https://img.shields.io/crates/v/tpt-prompt-template.svg)](https://crates.io/crates/tpt-prompt-template)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Safe, typed templating for prompts with compile-time validation.

**When to use this crate:** you're building prompts by string substitution
and want `{{variable}}` placeholders validated (parse errors on malformed
templates) rather than silently producing a broken prompt.

## Features

- `std` (default): Enables standard library features
- Validation of `{{variable}}` syntax at parse time

## Usage

```rust
use tpt_prompt_template::PromptTemplate;

let tmpl = PromptTemplate::new("Hello {{name}}, topic: {{topic}}").unwrap();
let rendered = tmpl.render(&[("name", "Alice"), ("topic", "Rust")]);
assert_eq!(rendered, "Hello Alice, topic: Rust");
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
