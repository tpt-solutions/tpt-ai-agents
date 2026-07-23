//! The same tool-calling round trip as `examples/full_agent_loop.rs`, but
//! expressed with `tpt-agent-graph` instead of inline control flow:
//!
//! - Nodes (`CallModel`, `RunTools`) are named, testable steps instead of an
//!   `if let Some(tool_calls) = ...` block.
//! - `ToolRegistry` dispatches a tool call to its handler *by name*, which is
//!   the piece `#[tool]` alone doesn't give you: it generates a
//!   `<fn>_call(args_json)` function per tool, but nothing to look one up by
//!   name when there's more than one. This example registers two tools to
//!   make that dispatch real (not just a single hardcoded call).
//!
//! Run with `cargo run --example agent_graph_loop`.

use tpt_agent_graph::{AgentGraph, Node, NodeOutcome, ToolRegistry};
use tpt_ai_mock_server::{MockServer, RecordedResponse};
use tpt_llm_client_core::{ChatRequest, Message, SseClient, Tool, ToolFunction};
use tpt_tool_use_macros::tool;

#[tool]
fn get_weather(location: String) -> String {
    format!("{location}: 72F and sunny")
}

#[tool]
fn get_time(location: String) -> String {
    format!("{location}: 14:32 local time")
}

/// Context threaded through every node in the graph.
struct Context {
    client: SseClient,
    tools: ToolRegistry,
    tool_defs: Vec<Tool>,
    conversation: Vec<Message>,
    pending_tool_calls: Vec<(String, String, String)>, // (id, name, arguments)
    final_answer: Option<String>,
}

/// Sends the conversation so far to the model. If it responds with tool
/// calls, stash them and go to `run_tools`; otherwise record the final
/// answer and halt.
struct CallModel;
impl Node<Context> for CallModel {
    fn name(&self) -> &str {
        "call_model"
    }

    fn run(&self, ctx: &mut Context) -> NodeOutcome {
        // `Node::run` is synchronous by design (the trait stays generic and
        // `no_std`-friendly), but the LLM call is async. `block_in_place` +
        // `block_on` is safe here because `#[tokio::main]` defaults to the
        // multi-thread runtime; it would panic on a current-thread runtime.
        let response = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(ctx.client.send(&ChatRequest {
                model: "gpt-4o-mini".into(),
                messages: ctx.conversation.clone(),
                temperature: None,
                max_tokens: None,
                stream: None,
                tools: Some(ctx.tool_defs.clone()),
            }))
        })
        .expect("model request failed");

        let choice = &response.choices[0];
        ctx.conversation.push(choice.message.clone());

        if let Some(ref tool_calls) = choice.message.tool_calls {
            ctx.pending_tool_calls = tool_calls
                .iter()
                .map(|tc| {
                    (
                        tc.id.clone(),
                        tc.function.name.clone(),
                        tc.function.arguments.clone(),
                    )
                })
                .collect();
            NodeOutcome::goto("run_tools")
        } else {
            ctx.final_answer = Some(choice.message.text().to_string());
            NodeOutcome::Halt
        }
    }
}

/// Dispatches every pending tool call through the `ToolRegistry` by name,
/// appends each result as a `role: "tool"` message, then loops back to
/// `call_model` so it can see the results.
struct RunTools;
impl Node<Context> for RunTools {
    fn name(&self) -> &str {
        "run_tools"
    }

    fn run(&self, ctx: &mut Context) -> NodeOutcome {
        for (id, name, arguments) in std::mem::take(&mut ctx.pending_tool_calls) {
            println!("Dispatching tool `{name}` with args: {arguments}");
            let result = ctx
                .tools
                .dispatch(&name, &arguments)
                .unwrap_or_else(|err| format!("tool error: {err}"));
            println!("Tool result: {result}");
            ctx.conversation.push(Message::tool(&id, &result));
        }
        NodeOutcome::goto("call_model")
    }
}

#[tokio::main]
async fn main() {
    let tool_call_response = serde_json::json!({
        "id": "turn_1",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": null,
                "tool_calls": [
                    {
                        "index": 0,
                        "id": "call_weather",
                        "type": "function",
                        "function": { "name": "get_weather", "arguments": "{\"location\":\"Austin\"}" }
                    },
                    {
                        "index": 1,
                        "id": "call_time",
                        "type": "function",
                        "function": { "name": "get_time", "arguments": "{\"location\":\"Austin\"}" }
                    }
                ]
            },
            "finish_reason": "tool_calls"
        }]
    });
    let final_answer = r#"{"id":"turn_2","choices":[{"index":0,"message":{"role":"assistant","content":"It's 72F and sunny in Austin at 14:32 local time — good conditions for the outdoor event."},"finish_reason":"stop"}]}"#;

    let mut server = MockServer::new();
    server.add_response(RecordedResponse::json_response(
        &tool_call_response.to_string(),
    ));
    server.add_response(RecordedResponse::json_response(final_answer));
    let addr = server.start().await.unwrap();

    let mut tools = ToolRegistry::new();
    tools.register("get_weather", |args| {
        get_weather_call(args).map_err(|e| e.to_string())
    });
    tools.register("get_time", |args| {
        get_time_call(args).map_err(|e| e.to_string())
    });

    let tool_defs = vec![
        Tool {
            tool_type: "function".into(),
            function: ToolFunction {
                name: "get_weather".into(),
                description: Some("Get the current weather for a location".into()),
                parameters: serde_json::from_str(get_weather_schema()).unwrap(),
            },
        },
        Tool {
            tool_type: "function".into(),
            function: ToolFunction {
                name: "get_time".into(),
                description: Some("Get the current local time for a location".into()),
                parameters: serde_json::from_str(get_time_schema()).unwrap(),
            },
        },
    ];

    let mut graph = AgentGraph::new("call_model");
    graph.add_node(CallModel);
    graph.add_node(RunTools);

    let mut ctx = Context {
        client: SseClient::openai(&format!("http://{addr}/v1"), "sk-mock"),
        tools,
        tool_defs,
        conversation: vec![Message::user(
            "Is it a good day for an outdoor team event in Austin?",
        )],
        pending_tool_calls: Vec::new(),
        final_answer: None,
    };

    let path = graph.run(&mut ctx).expect("graph execution failed");
    println!("Path: {path:?}");
    println!("Final answer: {}", ctx.final_answer.unwrap());
}
