//! Interactive chat loop over `tpt-llm-client-core` + `tpt-agent-memory`.
//!
//! Runs a simple REPL: you type a message, the client sends it to a
//! mock LLM server (or a real one via `--api-key`), and the response
//! is printed. Conversation history is stored in `tpt-agent-memory`.
//!
//! Usage:
//!   cargo run --example agent_cli
//!   cargo run --example agent_cli -- --api-key sk-... --base-url https://api.openai.com/v1

use tpt_agent_memory::{MemoryEntry, MemoryStore, SearchQuery};
use tpt_ai_mock_server::{MockServer, RecordedResponse};
use tpt_llm_client_core::{ChatRequest, Message, SseClient};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut api_key = String::new();
    let mut base_url = String::new();
    let mut model = String::from("gpt-4o-mini");

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--api-key" => {
                i += 1;
                api_key = args.get(i).cloned().unwrap_or_default();
            }
            "--base-url" => {
                i += 1;
                base_url = args.get(i).cloned().unwrap_or_default();
            }
            "--model" => {
                i += 1;
                model = args.get(i).cloned().unwrap_or_default();
            }
            _ => {}
        }
        i += 1;
    }

    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");

    if base_url.is_empty() {
        // Start a mock server
        let addr = rt.block_on(async {
            let mut server = MockServer::new();
            server.add_response(RecordedResponse::json_response(
                r#"{"id":"mock","choices":[{"index":0,"message":{"role":"assistant","content":"[mock] I'm a simulated LLM. This is a demo of the agent-cli example."},"finish_reason":"stop"}]}"#,
            ));
            server.start().await.expect("failed to start mock server")
        });
        base_url = format!("http://{addr}/v1");
        api_key = "sk-mock".into();
        println!("[mock server started on {addr}]");
    }

    let client = SseClient::openai(&base_url, &api_key);
    let mut memory = MemoryStore::new();
    let mut history: Vec<Message> = Vec::new();

    println!("tpt-agent-cli (type 'quit' to exit)");
    println!("Model: {model}");
    println!();

    loop {
        print!("You: ");
        use std::io::Write;
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" || input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        memory.insert(MemoryEntry::new(
            &format!("user: {input}"),
            &["conversation"],
        ));

        let context = memory.search(&SearchQuery::new(input));
        let context_str = if context.is_empty() {
            String::new()
        } else {
            format!(
                "\n[relevant context: {}]",
                context
                    .iter()
                    .map(|e| e.content.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        };

        history.push(Message {
            role: tpt_llm_client_core::Role::User,
            content: Some(format!("{input}{context_str}")),
            tool_calls: None,
            tool_call_id: None,
        });

        let request = ChatRequest {
            model: model.clone(),
            messages: history.clone(),
            temperature: None,
            max_tokens: None,
            stream: None,
            tools: None,
        };

        match rt.block_on(client.send(&request)) {
            Ok(response) => {
                let reply = response.choices[0].message.text();
                println!("Assistant: {reply}");
                history.push(Message {
                    role: tpt_llm_client_core::Role::Assistant,
                    content: Some(reply.to_string()),
                    tool_calls: None,
                    tool_call_id: None,
                });
                memory.insert(MemoryEntry::new(
                    &format!("assistant: {reply}"),
                    &["conversation"],
                ));
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
        println!();
    }
    println!("Goodbye!");
}
