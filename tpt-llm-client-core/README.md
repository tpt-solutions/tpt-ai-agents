# tpt-llm-client-core

[![CI](https://img.shields.io/badge/CI-passing-brightgreen)](https://github.com/tpt/tpt-ai-agents)
[![crates.io](https://img.shields.io/crates/v/tpt-llm-client-core.svg)](https://crates.io/crates/tpt-llm-client-core)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama.

## Features

- `std` (default): Enables networking via reqwest
- `async`: Alias for `std`
- `json-sse`: Enables SSE parsing
- `tool-use`: Enables tool-use integration via `tpt-prompt-template`

## Usage

```rust,ignore
use tpt_llm_client_core::SseClient;

#[tokio::main]
async fn main() {
    let client = SseClient::openai("https://api.openai.com/v1", "sk-...");
    let mut stream = client.chat_completion("Hello").await.unwrap();
    while let Some(chunk) = stream.next().await {
        println!("{:?}", chunk);
    }
}
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
