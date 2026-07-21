# tpt-vector-store-traits

[![crates.io](https://img.shields.io/crates/v/tpt-vector-store-traits.svg)](https://crates.io/crates/tpt-vector-store-traits)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Standardized async traits for Qdrant, Milvus, pgvector.

## Features

- `std` (default): Enables standard library features
- `json-sse`: Enables JSON payload support

## Usage

```rust,ignore
use tpt_vector_store_traits::{VectorStore, Query, SearchResult};

struct QdrantStore { /* ... */ }

impl VectorStore for QdrantStore {
    type Error = QdrantError;
    type Distance = CosineDistance;
    type Payload = serde_json::Value;

    async fn search(&self, query: &Query) -> Result<Vec<SearchResult<Self::Payload>>, Self::Error> {
        todo!()
    }
}
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
