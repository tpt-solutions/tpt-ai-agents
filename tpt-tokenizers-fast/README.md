# tpt-tokenizers-fast

[![crates.io](https://img.shields.io/crates/v/tpt-tokenizers-fast.svg)](https://crates.io/crates/tpt-tokenizers-fast)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Lightweight, `no_std`-compatible BPE tokenization optimized for edge deployment.

**When to use this crate:** you need to count or split tokens (e.g. to
respect a model's context window) without pulling in a large tokenizer
runtime, or in a `no_std`/embedded/WASM context.

## Features

- `std` (default): Enables standard library features
- Minimal allocations in hot path
- Applies learned BPE merges in priority order (not a 1:1 character lookup)

## Usage

```rust
use tpt_tokenizers_fast::{BpeTokenizer, Vocabulary};
use std::collections::BTreeMap;

let mut vocab = Vocabulary::new();
for (i, ch) in "helo, wrd!".chars().enumerate() {
    vocab.insert(&ch.to_string(), i as u32);
}
let tokenizer = BpeTokenizer::new(vocab, BTreeMap::new());
let tokens = tokenizer.encode("hello").unwrap();
```

Construct `Vocabulary` and the merge table from your model's tokenizer
files (this crate does not parse a specific file format for you).

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
