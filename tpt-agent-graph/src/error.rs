use alloc::string::String;
use core::fmt;

/// Errors produced while building or executing an [`AgentGraph`](crate::AgentGraph).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// A node's outcome named a node that was never added to the graph.
    UnknownNode(String),
    /// Execution exceeded the configured step budget without halting.
    ///
    /// Graphs are permitted to contain cycles (e.g. retry loops), so this is
    /// the mechanism that catches a node graph that never terminates.
    StepBudgetExceeded { limit: usize },
    /// The graph has no entry node registered under its configured entry name.
    MissingEntryNode(String),
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::UnknownNode(name) => {
                write!(f, "node `{name}` was not added to the graph")
            }
            GraphError::StepBudgetExceeded { limit } => {
                write!(f, "graph execution exceeded its step budget of {limit}")
            }
            GraphError::MissingEntryNode(name) => {
                write!(f, "entry node `{name}` was not added to the graph")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GraphError {}

/// Errors produced while dispatching a tool call through a [`ToolRegistry`](crate::ToolRegistry).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolDispatchError {
    /// No handler was registered under this tool name.
    UnknownTool(String),
    /// The handler itself returned an error (e.g. bad arguments, execution failure).
    Handler(String),
}

impl fmt::Display for ToolDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToolDispatchError::UnknownTool(name) => {
                write!(f, "no tool registered under the name `{name}`")
            }
            ToolDispatchError::Handler(msg) => write!(f, "tool handler failed: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ToolDispatchError {}
