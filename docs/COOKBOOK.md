# Cookbook

Common tasks mapped to the specific example files and crates involved.

## Build a RAG chatbot

**Crates:** `tpt-rag-pipeline`, `tpt-agent-memory`, `tpt-llm-client-core`, `tpt-tool-use-macros`, `tpt-embeddings-client`

1. **Chunk your documents** with `tpt-rag-pipeline`:
   ```rust
   use tpt_rag_pipeline::{Chunker, ChunkConfig};
   let chunks = Chunker::new(ChunkConfig::new(512, 50))
       .chunk(&document).unwrap();
   ```

2. **Generate embeddings** with `tpt-embeddings-client`:
   ```rust
   use tpt_embeddings_client::{OpenAiEmbeddings, EmbeddingRequest, EmbeddingsClient};
   let client = OpenAiEmbeddings::new("https://api.openai.com/v1", "sk-...");
   let response = client.embed(&EmbeddingRequest {
       model: "text-embedding-3-small".into(),
       inputs: chunks.iter().map(|c| c.text.clone()).collect(),
       dimensions: None,
   }).await.unwrap();
   ```

3. **Store and retrieve context** with `tpt-agent-memory`:
   ```rust
   use tpt_agent_memory::{MemoryStore, MemoryEntry, SearchQuery};
   let mut store = MemoryStore::new();
   for chunk in &chunks {
       store.insert(MemoryEntry::new(&chunk.text, &["document"]));
   }
   let context = store.search(&SearchQuery::new("user question"));
   ```

3. **Call the model** with `tpt-llm-client-core`, passing retrieved context into the prompt.

4. **Expose callable tools** with `tpt-tool-use-macros`:
   ```rust
   use tpt_tool_use_macros::tool;
   #[tool]
   fn search_documents(query: String) -> String { /* ... */ }
   ```

**See:** [`examples/full_agent_loop.rs`](../examples/full_agent_loop.rs), [`examples/rag_memory.rs`](../examples/rag_memory.rs)

## Add tool calling to an existing agent

**Crates:** `tpt-tool-use-macros`, `tpt-llm-client-core`

1. Annotate your function with `#[tool]`:
   ```rust
   use tpt_tool_use_macros::tool;

   /// Look up weather for a location.
   #[tool]
   fn get_weather(location: String) -> String {
       format!("{location}: 72F and sunny")
   }
   ```

2. The macro generates three items:
   - `get_weather_schema()` — JSON schema string for the function
   - `GetWeatherArgs` — `Serialize + Deserialize` struct for the parameters
   - `get_weather_call(args_json)` — deserializes JSON and invokes the function

3. Parse the model's tool-call response, pass the arguments to the generated dispatcher, and feed the result back to the model.

**See:** [`examples/full_agent_loop.rs`](../examples/full_agent_loop.rs) (lines 86-92)

## Swap in a different vector store

**Crates:** `tpt-vector-store-traits`, one adapter crate (`tpt-vector-store-qdrant`, `tpt-vector-store-pgvector`, `tpt-vector-store-milvus`)

1. Implement `tpt_vector_store_traits::VectorStore` for your backend, or use an existing adapter:

   | Backend | Crate | Status |
   |---------|-------|--------|
   | Qdrant | `tpt-vector-store-qdrant` | Production-ready |
    | pgvector | `tpt-vector-store-pgvector` | Beta (compiles, not yet published) |
   | Milvus | `tpt-vector-store-milvus` | Coming soon |

2. Wire it into `tpt-agent-memory` via `VectorBackedMemoryStore`:
   ```rust
   use tpt_agent_memory::VectorBackedMemoryStore;
   use tpt_vector_store_qdrant::QdrantVectorStore;

   let backend = QdrantVectorStore::new("http://localhost:6334", "my_collection").await?;
   let mut memory = VectorBackedMemoryStore::new(backend);
   memory.insert(entry.with_embedding(vec)).await?;
   let results = memory.search_semantic(query_embedding, 10).await?;
   ```

**See:** [`examples/rag_with_qdrant.rs`](../examples/rag_with_qdrant.rs)

## Stream chat completions

**Crate:** `tpt-llm-client-core`

```rust
use futures::StreamExt;
use tpt_llm_client_core::{SseClient, ChatRequest, Message, Role};

let client = SseClient::openai("https://api.openai.com/v1", "sk-...");
let mut stream = client.stream(&ChatRequest {
    model: "gpt-4o-mini".into(),
    messages: vec![Message { role: Role::User, content: "Hello".into() }],
    temperature: None,
    max_tokens: None,
    stream: Some(true),
}).await.unwrap();

while let Some(chunk) = stream.next().await {
    let chunk = chunk.unwrap();
    for choice in &chunk.choices {
        if let Some(content) = &choice.delta.content {
            print!("{content}");
        }
    }
}
```

**See:** [`examples/streaming_chat.rs`](../examples/streaming_chat.rs)

## Test your integration without a real API

**Crate:** `tpt-ai-mock-server`

```rust
use tpt_ai_mock_server::{MockServer, RecordedResponse};
use tpt_llm_client_core::{SseClient, ChatRequest, Message, Role};

let mut server = MockServer::new();
server.add_response(RecordedResponse::json_response(
    r#"{"id":"mock","choices":[{"index":0,"message":{"role":"assistant","content":"Hello!"},"finish_reason":"stop"}]}"#,
));
let addr = server.start().await.unwrap();

let client = SseClient::openai(&format!("http://{addr}/v1"), "sk-mock");
let response = client.send(&ChatRequest {
    model: "test".into(),
    messages: vec![Message { role: Role::User, content: "Hi".into() }],
    temperature: None,
    max_tokens: None,
    stream: None,
}).await.unwrap();
```

**See:** [`examples/full_agent_loop.rs`](../examples/full_agent_loop.rs), [`examples/streaming_chat.rs`](../examples/streaming_chat.rs)

## Run a model evaluation

**Crate:** `tpt-eval-harness`

```rust
use tpt_eval_harness::{EvalHarness, EvalConfig, EvalSample};

let harness = EvalHarness::new(EvalConfig::default());
let samples = vec![
    EvalSample::new("What is 2+2?", "4", "4"),
    EvalSample::new("Capital of France?", "Paris", "Lyon"),
];
let metrics = harness.run(&samples);
println!("accuracy: {}", metrics.accuracy());
```

Load from JSONL with `harness.run_file("dataset.jsonl")` (requires `std`).

**See:** crate docs at `tpt-eval-harness/src/lib.rs`

## Tokenize text without a large runtime

**Crate:** `tpt-tokenizers-fast`

BPE tokenization that works in `no_std` environments:

```rust
use tpt_tokenizers_fast::BpeTokenizer;

let vocab = vec![("<unk>".into(), 0), ("hello".into(), 1), ("world".into(), 2)];
let merges = vec![("h e".into(), 0), ("l l".into(), 1)];
let tokenizer = BpeTokenizer::new(vocab, merges);
let tokens = tokenizer.encode("hello");
```

**See:** crate docs at `tpt-tokenizers-fast/src/lib.rs`

## Render typed prompt templates

**Crate:** `tpt-prompt-template`

```rust
use tpt_prompt_template::PromptTemplate;

let tmpl = PromptTemplate::new("You are a helpful assistant. User asks: {{question}}").unwrap();
assert_eq!(tmpl.variables().len(), 1);
let rendered = tmpl.render(&[("question", "What is Rust?")]);
assert!(rendered.contains("What is Rust?"));
```

**See:** crate docs at `tpt-prompt-template/src/lib.rs`

## Run a local ONNX model

**Crate:** `tpt-onnx-runtime-utils`

```rust
use tpt_onnx_runtime_utils::{OnnxSession, Tensor};

let session = OnnxSession::from_file("model.onnx")?;
let input = Tensor::new("input", &[1, 3, 224, 224], vec![0.0f32; 150528]);
let output = session.run(&[input])?;
```

Requires the `std` feature (uses `tract-onnx` for pure-Rust inference, no system C toolchain needed).

**See:** crate docs at `tpt-onnx-runtime-utils/src/lib.rs`

## Orchestrate a multi-step agent loop

**Crates:** `tpt-agent-graph`, `tpt-llm-client-core`, `tpt-tool-use-macros`

Use `tpt-agent-graph` to express "call the model, run any requested tools,
call the model again" as named, testable nodes instead of inline control
flow, and `ToolRegistry` to dispatch a tool call to its handler by name
(closing the gap where `#[tool]` alone gives you a `<fn>_call` per tool but no
way to look one up by name once there's more than one):

```rust
use tpt_agent_graph::{AgentGraph, Node, NodeOutcome, ToolRegistry};

struct CallModel;
impl Node<Context> for CallModel {
    fn name(&self) -> &str { "call_model" }
    fn run(&self, ctx: &mut Context) -> NodeOutcome {
        // send ctx.conversation to the model; if it returned tool_calls,
        // stash them on ctx and continue to "run_tools", otherwise halt
        // with the final answer.
        if ctx.has_pending_tool_calls() {
            NodeOutcome::goto("run_tools")
        } else {
            NodeOutcome::Halt
        }
    }
}

let mut tools = ToolRegistry::new();
tools.register("get_weather", |args| get_weather_call(args).map_err(|e| e.to_string()));

let mut graph = AgentGraph::new("call_model");
graph.add_node(CallModel);
// ...add a "run_tools" node that calls tools.dispatch(name, args_json)
// for each pending call, then loops back to "call_model".

let path = graph.run(&mut ctx).unwrap();
```

Each node's `run()` returns the name of the next node directly
(`NodeOutcome::Goto`/`Halt`) rather than the graph resolving edges from a
separate topology table — a deliberately thinner design than a full
LangGraph-style graph runtime (see `rust-langgraph` on crates.io if you need
checkpoints, conditional-edge tables, or a Pregel-style execution model).
Cycles are allowed (e.g. retry loops) and guarded by a step budget
(`AgentGraph::with_step_budget`, default 1000).

**See:** [`examples/agent_graph_loop.rs`](../examples/agent_graph_loop.rs), crate docs at `tpt-agent-graph/src/lib.rs`

## Deploy Qdrant for development

Use the provided `docker-compose.yml` at the workspace root:

```sh
docker compose up -d qdrant
```

This starts Qdrant on `http://localhost:6334` (gRPC) and `http://localhost:6333` (HTTP).

Then create a collection and run the Qdrant example:

```sh
curl -X PUT http://localhost:6333/collections/example_docs \
  -H 'Content-Type: application/json' \
  -d '{"vectors": {"size": 3, "distance": "Cosine"}}'
cargo run --example rag_with_qdrant
```

**See:** [`docker-compose.yml`](../docker-compose.yml), [`examples/rag_with_qdrant.rs`](../examples/rag_with_qdrant.rs)
