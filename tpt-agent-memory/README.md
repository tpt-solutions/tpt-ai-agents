# tpt-agent-memory

[![crates.io](https://img.shields.io/crates/v/tpt-agent-memory.svg)](https://crates.io/crates/tpt-agent-memory)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

In-memory and file-persistent graph memory for multi-turn agent state, with
keyword and embedding-based semantic search.

**When to use this crate:** you need to remember things across turns of a
conversation or agent loop — facts, prior tool results, retrieved
documents — and search back over them later, either by keyword or (if you
attach embeddings from your own embedding model) by similarity.

## Features

- `std` (default): Enables standard library features, including file
  persistence (`MemoryStore::save_to_file`/`load_from_file`) and
  `ConcurrentMemoryStore` for multi-threaded access
- Keyword (substring) search always available; cosine-similarity semantic
  search when entries carry an embedding
- `vector-store`: Enables `VectorBackedMemoryStore`, which delegates
  embedding search to a real `tpt-vector-store-traits::VectorStore` backend
  (Qdrant, pgvector, etc.) instead of scanning in-memory — for when you have
  too many entries to scan linearly

## Usage

```rust
use tpt_agent_memory::{MemoryEntry, MemoryStore, SearchQuery};

let mut store = MemoryStore::new();
store.insert(MemoryEntry::new("user prefers dark mode", &["pref"]));

// Keyword search:
let results = store.search(&SearchQuery::new("dark mode"));

// Semantic search, once you have an embedding for the entry and the query:
store.insert(MemoryEntry::new("likes minimal UIs", &["pref"]).with_embedding(vec![0.1, 0.9]));
let results = store.search(&SearchQuery::new("").with_embedding(vec![0.1, 0.9]));
```

With the `vector-store` feature, swap `MemoryStore` for `VectorBackedMemoryStore<S>`
where `S` implements `tpt_vector_store_traits::VectorStore` (see
[`examples/qdrant_adapter.rs`](../examples/qdrant_adapter.rs) for a reference
adapter shape) to scale semantic search past what a linear scan can handle:

```rust,ignore
use tpt_agent_memory::{MemoryEntry, VectorBackedMemoryStore};

let mut store = VectorBackedMemoryStore::new(my_qdrant_adapter);
store.insert(MemoryEntry::new("likes minimal UIs", &["pref"]).with_embedding(vec![0.1, 0.9])).await?;
let results = store.search_semantic(vec![0.1, 0.9], 10).await?;
```

See [GETTING_STARTED.md](../GETTING_STARTED.md) for a full agent-loop example.

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
