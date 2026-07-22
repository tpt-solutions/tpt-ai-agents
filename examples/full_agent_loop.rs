//! End-to-end example wiring four crates into one agent turn:
//!
//! - `tpt-ai-mock-server` stands in for a real LLM API so this example runs
//!   offline and is safe to execute in CI (no API key needed).
//! - `tpt-llm-client-core` talks to it exactly as it would talk to
//!   OpenAI/Anthropic/Ollama, with automatic retry on transient failures.
//! - `tpt-tool-use-macros` exposes a plain Rust function as a callable tool
//!   with generated JSON-schema + argument (de)serialization.
//! - `tpt-rag-pipeline` chunks a small knowledge base and `tpt-agent-memory`
//!   stores/retrieves it, providing context for the final answer.
//!
//! Run with `cargo run --example full_agent_loop`.
//!
//! Note: this mock server only returns pre-recorded responses — it does not
//! implement real tool-calling semantics (e.g. OpenAI's `tool_calls` field).
//! To keep the demo self-contained, the mock's first-turn reply embeds the
//! tool call as `CALL_TOOL:<json args>` in the message content, and this
//! example parses that convention. A real provider would use its native
//! tool-calling response shape instead.

use tpt_agent_memory::{MemoryEntry, MemoryStore, SearchQuery};
use tpt_ai_mock_server::{MockServer, RecordedResponse};
use tpt_llm_client_core::{ChatRequest, Message, Role, SseClient};
use tpt_rag_pipeline::{ChunkConfig, Chunker};
use tpt_tool_use_macros::tool;

/// The tool the model can call. `#[tool]` generates `get_weather_schema()`
/// (JSON schema) and `get_weather_call(args_json)` (deserializes args and
/// invokes this function) alongside it.
#[tool]
fn get_weather(location: String) -> String {
    format!("{location}: 72F and sunny")
}

#[tokio::main]
async fn main() {
    // 1. Index a small knowledge base with the RAG pipeline + agent memory,
    //    so the final answer can be grounded in retrieved context.
    let knowledge_base = "Our office policy: remote employees should check \
        local weather before scheduling outdoor team events. Always confirm \
        conditions the morning of the event.";
    let chunker = Chunker::new(ChunkConfig::new(20, 5));
    let mut memory = MemoryStore::new();
    for chunk in chunker.chunk(knowledge_base).expect("chunking failed") {
        memory.insert(MemoryEntry::new(&chunk.text, &["policy"]));
    }
    let context = memory.search(&SearchQuery::new("weather"));
    println!(
        "Retrieved {} relevant memory entr{} for context",
        context.len(),
        if context.len() == 1 { "y" } else { "ies" }
    );

    // 2. Start a mock LLM server and queue two turns: a tool-call request,
    //    then a final answer that references the tool result.
    let mut server = MockServer::new();
    server.add_response(RecordedResponse::json_response(
        r#"{"id":"turn_1","choices":[{"index":0,"message":{"role":"assistant","content":"CALL_TOOL:{\"location\":\"Austin\"}"},"finish_reason":"stop"}]}"#,
    ));
    server.add_response(RecordedResponse::json_response(
        r#"{"id":"turn_2","choices":[{"index":0,"message":{"role":"assistant","content":"It's 72F and sunny in Austin, consistent with our office policy of checking before outdoor events. You're clear to schedule it."},"finish_reason":"stop"}]}"#,
    ));
    let addr = server.start().await.unwrap();

    // 3. Talk to it via the real client crate, exactly as with a live provider.
    let client = SseClient::openai(&format!("http://{addr}/v1"), "sk-mock");
    let mut conversation = vec![Message {
        role: Role::User,
        content: "Is it a good day for an outdoor team event in Austin?".into(),
    }];

    let turn_1 = client
        .send(&ChatRequest {
            model: "gpt-4o-mini".into(),
            messages: conversation.clone(),
            temperature: None,
            max_tokens: None,
            stream: None,
        })
        .await
        .expect("turn 1 request failed");
    let reply = &turn_1.choices[0].message.content;
    println!("Model (turn 1): {reply}");

    // 4. Detect and execute the tool call via the macro-generated dispatcher.
    let tool_result = if let Some(args_json) = reply.strip_prefix("CALL_TOOL:") {
        let result = get_weather_call(args_json).expect("invalid tool arguments");
        println!("Tool result: {result}");
        result
    } else {
        String::new()
    };

    // 5. Send the tool result back for a final, grounded answer.
    conversation.push(Message {
        role: Role::Assistant,
        content: reply.clone(),
    });
    conversation.push(Message {
        role: Role::User,
        content: format!("Tool result: {tool_result}"),
    });
    let turn_2 = client
        .send(&ChatRequest {
            model: "gpt-4o-mini".into(),
            messages: conversation,
            temperature: None,
            max_tokens: None,
            stream: None,
        })
        .await
        .expect("turn 2 request failed");
    println!("Model (turn 2): {}", turn_2.choices[0].message.content);
}
