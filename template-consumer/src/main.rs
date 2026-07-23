use tpt_agent_memory::{MemoryEntry, MemoryStore, SearchQuery};
use tpt_llm_client_core::{ChatRequest, Message, SseClient};
use tpt_tool_use_macros::tool;

#[tool]
fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[tokio::main]
async fn main() {
    // Configure the client — swap for a real provider or use tpt-ai-mock-server
    let client = SseClient::openai("http://localhost:8080/v1", "sk-...");
    let mut memory = MemoryStore::new();
    let mut history: Vec<Message> = Vec::new();

    // Store an initial fact
    memory.insert(MemoryEntry::new("User prefers concise answers", &["preference"]));

    // Build a message with relevant context
    let context = memory.search(&SearchQuery::new("preferences"));
    let context_str = if context.is_empty() {
        String::new()
    } else {
        format!(
            "\n[context: {}]",
            context
                .iter()
                .map(|e| e.content.as_str())
                .collect::<Vec<_>>()
                .join("; ")
        )
    };

    history.push(Message::user(&format!("Hello!{context_str}")));

    match client.send(&ChatRequest {
        model: "gpt-4o-mini".into(),
        messages: history,
        temperature: None,
        max_tokens: None,
        stream: None,
        tools: None,
    }).await {
        Ok(response) => println!("{}", response.choices[0].message.text()),
        Err(e) => eprintln!("Error: {e}"),
    }
}
