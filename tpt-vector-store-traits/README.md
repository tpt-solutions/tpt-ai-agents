# tpt-vector-store-traits

[![crates.io](https://img.shields.io/crates/v/tpt-vector-store-traits.svg)](https://crates.io/crates/tpt-vector-store-traits)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Standardized async `VectorStore` trait for implementing vector database
backend adapters.

**When to use this crate:** you want your code to be portable across
vector database backends. This crate ships the trait only — **no Qdrant,
Milvus, or pgvector adapter is included**; implement `VectorStore` for your
own client (see [`examples/qdrant_adapter.rs`](../examples/qdrant_adapter.rs)
in the workspace root for the shape of a real adapter). Once implemented,
`tpt-agent-memory`'s `VectorBackedMemoryStore` (behind its `vector-store`
feature) can use it directly for embedding search at scale, in place of
that crate's default in-memory cosine-similarity scan.

## Features

- `std` (default): Enables standard library features
- `json-sse`: Enables JSON payload support

## Usage

```rust,ignore
use tpt_vector_store_traits::{VectorStore, Query, SearchResult, CosineDistance};
use async_trait::async_trait;

struct QdrantStore { /* ... */ }

#[async_trait]
impl VectorStore for QdrantStore {
    type Error = QdrantError;
    type Distance = CosineDistance;
    type Payload = serde_json::Value;

    async fn search(&self, query: &Query) -> Result<Vec<SearchResult<Self::Payload>>, Self::Error> {
        todo!("call the Qdrant client")
    }
    // ...upsert, delete, info
}
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
