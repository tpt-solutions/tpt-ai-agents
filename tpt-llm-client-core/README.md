# tpt-llm-client-core

[![CI](https://github.com/tpt-solutions/tpt-ai-agents/actions/workflows/ci.yml/badge.svg)](https://github.com/tpt-solutions/tpt-ai-agents/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/tpt-llm-client-core.svg)](https://crates.io/crates/tpt-llm-client-core)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama, with
automatic retry/backoff on transient failures (connection errors, HTTP 429,
HTTP 5xx).

**When to use this crate:** you want one client API that works against
OpenAI, Anthropic, or a local Ollama server, without hand-rolling each
provider's request/response shape or SSE parsing.

## Features

- `std` (default): Enables networking via reqwest
- `async`: Alias for `std`
- `json-sse`: Enables SSE parsing
- `tool-use`: Enables tool-use integration via `tpt-prompt-template`

## Usage

```rust,no_run
use tpt_llm_client_core::{SseClient, ChatRequest, Message, Role};

# async fn run() {
let client = SseClient::openai("https://api.openai.com/v1", "sk-...");
let response = client.send(&ChatRequest {
    model: "gpt-4o-mini".into(),
    messages: vec![Message { role: Role::User, content: "Hello".into() }],
    temperature: None,
    max_tokens: None,
    stream: None,
}).await.unwrap();
println!("{}", response.choices[0].message.content);
# }
```

Or streaming, via `client.stream(&request)` which returns a
`futures::Stream<Item = Result<StreamChunk>>`. Test against
`tpt-ai-mock-server` instead of a live API — see its README and
[`examples/full_agent_loop.rs`](../examples/full_agent_loop.rs) in the
workspace root.

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
