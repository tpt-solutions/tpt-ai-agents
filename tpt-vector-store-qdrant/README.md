# tpt-vector-store-qdrant

[![crates.io](https://img.shields.io/crates/v/tpt-vector-store-qdrant.svg)](https://crates.io/crates/tpt-vector-store-qdrant)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

A real [`tpt-vector-store-traits::VectorStore`](../tpt-vector-store-traits)
adapter backed by [Qdrant](https://qdrant.tech), via the official
[`qdrant-client`](https://crates.io/crates/qdrant-client) crate.

**When to use this crate:** you're using `tpt-agent-memory`'s
`VectorBackedMemoryStore` (or `tpt-vector-store-traits::VectorStore`
directly) and want a working Qdrant backend instead of writing your own
adapter from scratch.

The target collection must already exist in Qdrant (with a vector size and
distance metric of your choosing) — this crate does not create collections,
only reads and writes points in one.

## Features

- `std` (default): Enables the client itself (networking via
  `qdrant-client`/`tonic`)
- `async`: Alias for `std`

## Usage

```rust,no_run
use tpt_vector_store_qdrant::QdrantVectorStore;
use tpt_vector_store_traits::{Query, VectorStore};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let store = QdrantVectorStore::new("http://localhost:6334", "my_collection").await?;

store
    .upsert(&["doc1"], &[&[0.1, 0.2, 0.3]], &[None])
    .await?;

let results = store
    .search(&Query {
        vector: vec![0.1, 0.2, 0.3],
        filter: None,
        limit: 5,
        offset: None,
        include_payload: Some(true),
    })
    .await?;
println!("{} results", results.len());
# Ok(())
# }
```

With `tpt-agent-memory`'s `vector-store` feature:

```rust,no_run
use tpt_agent_memory::VectorBackedMemoryStore;
use tpt_vector_store_qdrant::QdrantVectorStore;

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let backend = QdrantVectorStore::new("http://localhost:6334", "memory").await?;
let store = VectorBackedMemoryStore::new(backend);
# let _ = store;
# Ok(())
# }
```

Run a local Qdrant instance for testing with:

```sh
docker run -p 6334:6334 qdrant/qdrant
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
