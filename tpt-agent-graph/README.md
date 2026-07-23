# tpt-agent-graph

[![crates.io](https://img.shields.io/crates/v/tpt-agent-graph.svg)](https://crates.io/crates/tpt-agent-graph)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Thin state-machine/DAG orchestration for LLM agent turns.

**When to use this crate:** you're wiring `tpt-llm-client-core` +
`tpt-tool-use-macros` (+ optionally `tpt-agent-memory`) into a multi-step
agent loop — "call the model, run any requested tools, call the model again"
— and want that loop expressed as named, testable steps instead of inline
control flow, plus a way to route a tool call to its handler by name when
there's more than one tool.

This crate depends on none of those — it's generic over a caller-defined
context type and dispatches tools by plain string name, so it stays
`no_std`-compatible and composable with whatever client/memory types you
already have.

## Features

- `std` (default): Enables standard library features

## Usage

```rust
use tpt_agent_graph::{AgentGraph, Node, NodeOutcome, ToolRegistry};

struct Context {
    tools: ToolRegistry,
    last_result: Option<String>,
}

struct CallTool;
impl Node<Context> for CallTool {
    fn name(&self) -> &str { "call_tool" }
    fn run(&self, ctx: &mut Context) -> NodeOutcome {
        ctx.last_result = ctx.tools.dispatch("get_weather", r#"{"location":"Austin"}"#).ok();
        NodeOutcome::Halt
    }
}

let mut tools = ToolRegistry::new();
tools.register("get_weather", |_args| Ok("72F and sunny".to_string()));

let mut graph = AgentGraph::new("call_tool");
graph.add_node(CallTool);

let mut ctx = Context { tools, last_result: None };
graph.run(&mut ctx).unwrap();
```

`AgentGraph` allows cycles (e.g. a "retry the tool call" loop) guarded by a
step budget (`AgentGraph::with_step_budget`, default 1000) so a runaway graph
returns `GraphError::StepBudgetExceeded` instead of hanging.

See [`examples/agent_graph_loop.rs`](../examples/agent_graph_loop.rs) in the
workspace root for the full multi-tool version wired to
`tpt-llm-client-core`, `tpt-tool-use-macros`, and `tpt-ai-mock-server`.

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
