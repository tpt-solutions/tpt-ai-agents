//! Thin state-machine/DAG orchestration for LLM agent turns.
//!
//! Ties `tpt-llm-client-core` (structured tool calls), `tpt-tool-use-macros`
//! (`#[tool]`-generated handlers), and `tpt-agent-memory` together as a
//! reusable "tier 2" crate, rather than only existing as a one-off example.
//! It intentionally does not depend on any of them: [`AgentGraph`] is generic
//! over a caller-defined context type, and [`ToolRegistry`] dispatches on
//! plain strings, so this crate stays `no_std`-compatible and composable with
//! whatever client/memory types you already have.
//!
//! Two independent pieces:
//!
//! - [`AgentGraph`]: a named set of [`Node`]s, each of which reads/writes a
//!   shared context and returns the name of the next node to run (or halts).
//!   This is the orchestration loop — "call the model, then run tools, then
//!   call the model again" — made explicit and inspectable instead of living
//!   as inline control flow in an example.
//! - [`ToolRegistry`]: maps a tool name (as it appears in a model's
//!   `tool_calls[i].function.name`) to a handler. This closes a real gap:
//!   `#[tool]` generates a `<fn>_call(args_json)` function per tool but no
//!   way to look one up by name, so multi-tool consumers previously had to
//!   hand-write that `match` themselves.
//!
//! # Example
//!
//! ```
//! use tpt_agent_graph::{AgentGraph, Node, NodeOutcome, ToolRegistry};
//!
//! struct Context {
//!     tools: ToolRegistry,
//!     last_result: Option<String>,
//! }
//!
//! struct CallTool;
//! impl Node<Context> for CallTool {
//!     fn name(&self) -> &str {
//!         "call_tool"
//!     }
//!     fn run(&self, ctx: &mut Context) -> NodeOutcome {
//!         let result = ctx.tools.dispatch("get_weather", r#"{"location":"Austin"}"#);
//!         ctx.last_result = result.ok();
//!         NodeOutcome::Halt
//!     }
//! }
//!
//! let mut tools = ToolRegistry::new();
//! tools.register("get_weather", |_args| Ok("72F and sunny".to_string()));
//!
//! let mut graph = AgentGraph::new("call_tool");
//! graph.add_node(CallTool);
//!
//! let mut ctx = Context { tools, last_result: None };
//! graph.run(&mut ctx).unwrap();
//! assert_eq!(ctx.last_result.as_deref(), Some("72F and sunny"));
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod graph;
mod registry;

pub use error::{GraphError, ToolDispatchError};
pub use graph::{AgentGraph, Node, NodeOutcome, DEFAULT_STEP_BUDGET};
pub use registry::ToolRegistry;
