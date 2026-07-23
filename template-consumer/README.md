# {{ project_name }}

{{ project_description }}

## Quick start

```sh
cargo run
```

Configure the LLM endpoint by editing `src/main.rs` — swap `SseClient::openai(...)` for your provider of choice, or use `tpt-ai-mock-server` for offline testing.

## Crates used

- `tpt-llm-client-core` — streaming HTTP/SSE client for LLM APIs
- `tpt-agent-memory` — in-memory store with keyword search
- `tpt-tool-use-macros` — `#[tool]` attribute for exposing Rust functions as LLM tools
