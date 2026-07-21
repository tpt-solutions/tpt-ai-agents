# tpt-rag-pipeline

[![crates.io](https://img.shields.io/crates/v/tpt-rag-pipeline.svg)](https://crates.io/crates/tpt-rag-pipeline)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Chunking, embedding batching, and context-window management.

## Features

- `std` (default): Enables standard library features
- `full`: Enables all integrations

## Usage

```rust,ignore
use tpt_rag_pipeline::{Chunker, ChunkConfig};

let config = ChunkConfig::new(512, 50);
let mut chunker = Chunker::new(config);
let chunks = chunker.chunk("Your long document text here...").unwrap();
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
