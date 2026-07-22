# tpt-ai-mock-server

[![crates.io](https://img.shields.io/crates/v/tpt-ai-mock-server.svg)](https://crates.io/crates/tpt-ai-mock-server)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

A real local HTTP server for testing LLM integrations — records/replays
OpenAI-shaped JSON or SSE responses on `POST /v1/chat/completions` over an
actual TCP connection, so client code (e.g. `tpt-llm-client-core`) can be
tested exactly as it would run against a live provider, with no network
access or API key required.

**When to use this crate:** in tests or examples for anything that calls
`tpt-llm-client-core` (or any OpenAI-API-shaped client). Queue one or more
responses, start the server, point your client at its address.

## Features

- `std` (default): Enables standard library features
- Request schema validation (`model`/`messages` required, else `400`)
- Multiple queued responses per path served in FIFO order — script a
  multi-turn conversation (e.g. a tool-call turn then a final answer)

## Usage

```rust,no_run
use tpt_ai_mock_server::{MockServer, RecordedResponse};

# async fn run() {
let mut server = MockServer::new();
server.add_response(RecordedResponse::json_response(
    r#"{"id":"1","choices":[{"index":0,"message":{"role":"assistant","content":"hi"},"finish_reason":"stop"}]}"#,
));
let addr = server.start().await.unwrap();
// point an OpenAI-shaped client at http://{addr}/v1
# }
```

See [`examples/full_agent_loop.rs`](../examples/full_agent_loop.rs) in the
workspace root for a complete example driving `tpt-llm-client-core` against
this server.

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
