# Getting Started

`tpt-ai-agents` is a workspace of small, composable Rust crates for LLM/agent
infrastructure — an HTTP/SSE client, tool-calling macros, RAG chunking,
agent memory, tokenization, ONNX inference, and a mock server for testing.
Each crate works standalone; pull in only what you need.

These crates are not yet published to crates.io (see the root
[README.md](README.md) for publish status). Until then, depend on them via
path or git:

```toml
[dependencies]
tpt-llm-client-core = { git = "https://github.com/tpt-solutions/tpt-ai-agents", package = "tpt-llm-client-core" }
```

## Smallest possible example

Two dependency-free crates — templating and tokenization — with no network
or async runtime required:

```toml
[dependencies]
tpt-prompt-template = { git = "https://github.com/tpt-solutions/tpt-ai-agents" }
```

```rust
use tpt_prompt_template::PromptTemplate;

let template = PromptTemplate::new("Hello, {{name}}!").unwrap();
let rendered = template.render(&[("name", "world")]);
assert_eq!(rendered, "Hello, world!");
```

## A real agent turn

The most useful starting point is
[`examples/full_agent_loop.rs`](examples/full_agent_loop.rs), which wires
four crates together into one working agent turn:

- **`tpt-rag-pipeline`** chunks a knowledge base
- **`tpt-agent-memory`** stores and retrieves it
- **`tpt-tool-use-macros`** exposes a Rust function as a callable tool with
  a `#[tool]` attribute (JSON schema + argument (de)serialization generated
  for you)
- **`tpt-llm-client-core`** talks to the model (OpenAI/Anthropic/Ollama
  shapes, with automatic retry on transient failures)
- **`tpt-ai-mock-server`** stands in for the real API, so the example runs
  offline with no API key — useful for this demo, and for testing your own
  integration against `tpt-llm-client-core`

Clone the repo and run it directly:

```sh
git clone https://github.com/tpt-solutions/tpt-ai-agents
cd tpt-ai-agents
cargo run --example full_agent_loop
```

For a narrower example of just chunking + memory, see
[`examples/rag_memory.rs`](examples/rag_memory.rs).

## Talking to a real provider

Swap the mock server for a live one — `SseClient` speaks OpenAI, Anthropic,
and Ollama's native shapes:

```rust,no_run
use tpt_llm_client_core::{SseClient, ChatRequest, Message, Role};

# async fn run() {
let client = SseClient::openai("https://api.openai.com/v1", "sk-...");
// or: SseClient::anthropic("https://api.anthropic.com/v1", "sk-ant-...")
// or: SseClient::ollama("http://localhost:11434")

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

Requests retry automatically on connection errors, HTTP 429, and HTTP 5xx
with exponential backoff (3 attempts by default) — tune this via
`SseClient::with_retry`.

## Which crate do I need?

| I want to... | Crate |
|---|---|
| Call OpenAI/Anthropic/Ollama, streaming or not | `tpt-llm-client-core` |
| Give the model callable Rust functions | `tpt-tool-use-macros` |
| Store/search conversation state across turns | `tpt-agent-memory` |
| Chunk documents for retrieval | `tpt-rag-pipeline` |
| Standardize against a vector DB (traits only — no adapters shipped yet) | `tpt-vector-store-traits` |
| Write typed, validated prompt templates | `tpt-prompt-template` |
| Tokenize text without pulling in a large runtime | `tpt-tokenizers-fast` |
| Run a local ONNX model | `tpt-onnx-runtime-utils` |
| Test your integration without hitting a real API | `tpt-ai-mock-server` |
| Score a model against a fixed dataset | `tpt-eval-harness` |

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for how these compose and
the tier/dependency structure, and [CONTRIBUTING.md](CONTRIBUTING.md) for
adding a new crate.
