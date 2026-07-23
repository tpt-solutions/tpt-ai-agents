//! Minimal example: streaming chat completions with `tpt-llm-client-core`.
//!
//! Uses `tpt-ai-mock-server` to stand in for a real provider, so this runs
//! offline. Swap `SseClient::openai(...)` for `SseClient::anthropic(...)` or
//! `SseClient::ollama(...)` to talk to a different provider — the streaming
//! API is identical.
//!
//! Run with `cargo run --example streaming_chat`.

use futures::StreamExt;
use tpt_ai_mock_server::{MockServer, RecordedResponse, SseChunk};
use tpt_llm_client_core::{ChatRequest, Message, Role, SseClient};

#[tokio::main]
async fn main() {
    // Queue an SSE response shaped like OpenAI's streaming chat completion
    // chunks: one chunk per word, then a final chunk carrying `finish_reason`.
    let mut server = MockServer::new();
    let words = [
        "Streaming ",
        "responses ",
        "arrive ",
        "token ",
        "by ",
        "token.",
    ];
    let mut chunks: Vec<SseChunk> = words
        .iter()
        .map(|word| SseChunk {
            event_type: None,
            data: format!(
                r#"{{"id":"turn_1","choices":[{{"index":0,"delta":{{"content":"{word}"}},"finish_reason":null}}]}}"#
            ),
        })
        .collect();
    chunks.push(SseChunk {
        event_type: None,
        data: r#"{"id":"turn_1","choices":[{"index":0,"delta":{},"finish_reason":"stop"}]}"#.into(),
    });
    server.add_response(RecordedResponse { chunks });
    let addr = server.start().await.unwrap();

    let client = SseClient::openai(&format!("http://{addr}/v1"), "sk-mock");
    let request = ChatRequest {
        model: "gpt-4o-mini".into(),
        messages: vec![Message {
            role: Role::User,
            content: "Tell me about streaming.".into(),
        }],
        temperature: None,
        max_tokens: None,
        stream: Some(true),
    };

    let mut stream = client
        .stream(&request)
        .await
        .expect("stream request failed");
    print!("Model: ");
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.expect("stream error");
        for choice in &chunk.choices {
            if let Some(content) = &choice.delta.content {
                print!("{content}");
            }
        }
    }
    println!();
}
