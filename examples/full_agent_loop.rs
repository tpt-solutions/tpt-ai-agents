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
//! This example demonstrates the real tool-calling flow:
//! 1. Send a request with tool definitions
//! 2. Receive a response with `tool_calls` (structured, not text hacks)
//! 3. Execute the tool via `#[tool]`-generated dispatch
//! 4. Send the result back as a `role: "tool"` message

use tpt_agent_memory::{MemoryEntry, MemoryStore, SearchQuery};
use tpt_ai_mock_server::{MockServer, RecordedResponse};
use tpt_llm_client_core::{ChatRequest, Message, SseClient, Tool, ToolFunction};
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

    // 2. Start a mock LLM server and queue two turns:
    //    Turn 1: model responds with a real tool_calls structure
    //    Turn 2: model gives a final answer referencing the tool result
    let tool_call_response = serde_json::json!({
        "id": "turn_1",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": null,
                "tool_calls": [{
                    "index": 0,
                    "id": "call_abc123",
                    "type": "function",
                    "function": {
                        "name": "get_weather",
                        "arguments": "{\"location\":\"Austin\"}"
                    }
                }]
            },
            "finish_reason": "tool_calls"
        }]
    });
    let final_answer = r#"{"id":"turn_2","choices":[{"index":0,"message":{"role":"assistant","content":"It's 72F and sunny in Austin, consistent with our office policy of checking before outdoor events. You're clear to schedule it."},"finish_reason":"stop"}]}"#;

    let mut server = MockServer::new();
    server.add_response(RecordedResponse::json_response(
        &tool_call_response.to_string(),
    ));
    server.add_response(RecordedResponse::json_response(final_answer));
    let addr = server.start().await.unwrap();

    // 3. Talk to it via the real client crate, exactly as with a live provider.
    let client = SseClient::openai(&format!("http://{addr}/v1"), "sk-mock");
    let mut conversation = vec![Message::user(
        "Is it a good day for an outdoor team event in Austin?",
    )];

    // 4. Send the request WITH tool definitions.
    let turn_1 = client
        .send(&ChatRequest {
            model: "gpt-4o-mini".into(),
            messages: conversation.clone(),
            temperature: None,
            max_tokens: None,
            stream: None,
            tools: Some(vec![Tool {
                tool_type: "function".into(),
                function: ToolFunction {
                    name: "get_weather".into(),
                    description: Some("Get the current weather for a location".into()),
                    parameters: serde_json::from_str(get_weather_schema()).unwrap(),
                },
            }]),
        })
        .await
        .expect("turn 1 request failed");

    // 5. Check if the model requested a tool call (structured, not text hack).
    let choice = &turn_1.choices[0];
    if let Some(ref tool_calls) = choice.message.tool_calls {
        let tool_call = &tool_calls[0];
        println!(
            "Model requested tool: {} with args: {}",
            tool_call.function.name, tool_call.function.arguments
        );

        // 6. Execute the tool via the macro-generated dispatcher.
        let result =
            get_weather_call(&tool_call.function.arguments).expect("invalid tool arguments");
        println!("Tool result: {result}");

        // 7. Send the tool result back as a role:"tool" message.
        conversation.push(choice.message.clone());
        conversation.push(Message::tool(&tool_call.id, &result));

        let turn_2 = client
            .send(&ChatRequest {
                model: "gpt-4o-mini".into(),
                messages: conversation,
                temperature: None,
                max_tokens: None,
                stream: None,
                tools: None,
            })
            .await
            .expect("turn 2 request failed");
        println!("Model (turn 2): {}", turn_2.choices[0].message.text());
    } else {
        println!("Model: {}", choice.message.text());
    }
}
