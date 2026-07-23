# tpt-embeddings-client

[![Crates.io](https://img.shields.io/crates/v/tpt-embeddings-client.svg)](https://crates.io/crates/tpt-embeddings-client)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

Unified async client for OpenAI, Cohere, and custom embedding APIs.

Part of the [`tpt-ai-agents`](https://github.com/tpt-solutions/tpt-ai-agents) workspace.

## Features

- **Trait-based**: implement `EmbeddingsClient` for any provider
- **OpenAI**: ready-made adapter for `/v1/embeddings` (text-embedding-3, etc.)
- **Cohere**: ready-made adapter for `/v1/embed` (embed-v3, etc.)
- **Batch support**: send multiple texts in a single request
- **Token tracking**: provider-reported usage in `EmbeddingMeta`

## Quick Start

```toml
[dependencies]
tpt-embeddings-client = "0.1"
```

```rust
use tpt_embeddings_client::{OpenAiEmbeddings, EmbeddingRequest, EmbeddingsClient};

# async fn run() -> Result<(), tpt_embeddings_client::Error> {
let client = OpenAiEmbeddings::new("https://api.openai.com/v1", "sk-...");
let response = client.embed(&EmbeddingRequest {
    model: "text-embedding-3-small".into(),
    inputs: vec!["hello world".into()],
    dimensions: None,
}).await?;
assert_eq!(response.embeddings.len(), 1);
# Ok(())
# }
```

## Feature Flags

| Feature   | Default | Description                                    |
|-----------|---------|------------------------------------------------|
| `std`     | yes     | Enables reqwest networking + OpenAI/Cohere      |
| `async`   | yes     | Alias for `std`                                |
| `json-sse`| yes     | Reserved (no-op)                               |

## License

MIT OR Apache-2.0
