# tpt-rag-pipeline

[![crates.io](https://img.shields.io/crates/v/tpt-rag-pipeline.svg)](https://crates.io/crates/tpt-rag-pipeline)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Chunking, embedding batching, and context-window management for RAG
pipelines.

**When to use this crate:** you need to split documents into overlapping,
token-limited chunks before embedding/indexing them for retrieval.

Note: `EmbeddingBatch`/`ContextWindow` are basic data holders — this crate
does not call an embedding model itself; it batches vectors you already
computed (e.g. via `tpt-llm-client-core` or `tpt-onnx-runtime-utils`).

## Features

- `std` (default): Enables standard library features

## Usage

```rust
use tpt_rag_pipeline::{Chunker, ChunkConfig};

let config = ChunkConfig::new(512, 50);
let chunker = Chunker::new(config);
let chunks = chunker.chunk("Your long document text here...").unwrap();
```

See [`examples/rag_memory.rs`](../examples/rag_memory.rs) in the workspace
root for chunking combined with `tpt-agent-memory` storage/retrieval.

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
