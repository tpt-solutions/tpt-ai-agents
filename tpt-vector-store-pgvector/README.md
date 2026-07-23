# tpt-vector-store-pgvector

[![CI](https://github.com/tpt-solutions/tpt-ai-agents/actions/workflows/ci.yml/badge.svg)](https://github.com/tpt-solutions/tpt-ai-agents/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/tpt-vector-store-pgvector.svg)](https://crates.io/crates/tpt-vector-store-pgvector)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

PostgreSQL/pgvector adapter for [`tpt_vector_store_traits::VectorStore`](https://docs.rs/tpt-vector-store-traits).

## Overview

Implements the `VectorStore` trait against a PostgreSQL database with the
[pgvector](https://github.com/pgvector/pgvector) extension, so it can be
dropped straight into `tpt_agent_memory::VectorBackedMemoryStore` for
embedding search backed by a familiar, widely-deployed database.

## Features

- `std` (default): Enables the adapter (requires `sqlx` and `pg-vector`)
- `async`: Alias for `std`

## Requirements

- PostgreSQL 12+ with the `vector` extension enabled
- A table with the expected schema:

```sql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    embedding vector(384),
    payload JSONB
);
```

## Usage

```rust,no_run
use tpt_vector_store_pgvector::PgVectorStore;
use tpt_vector_store_traits::{Query, VectorStore};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let store = PgVectorStore::new(
    "postgres://user:pass@localhost/mydb",
    "documents",
    384,
).await?;

// Search
let results = store.search(&Query {
    vector: vec![0.1; 384],
    filter: None,
    limit: 5,
    offset: None,
    include_payload: Some(true),
}).await?;

// Upsert
store.upsert(
    &["doc1"],
    &[&[0.1; 384]],
    &[Some(&serde_json::json!({"source": "test"}))],
).await?;
# Ok(())
# }
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
