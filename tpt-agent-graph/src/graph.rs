use crate::error::GraphError;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// What a [`Node`] wants to happen after it runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeOutcome {
    /// Continue execution at the named node.
    Goto(String),
    /// Stop execution; this turn is complete.
    Halt,
}

impl NodeOutcome {
    /// Shorthand for `NodeOutcome::Goto(name.into())`.
    pub fn goto(name: impl Into<String>) -> Self {
        NodeOutcome::Goto(name.into())
    }
}

/// A single step in an [`AgentGraph`].
///
/// `C` is the caller-defined context threaded through every node (e.g. a
/// struct holding the conversation history, an LLM client, and a
/// [`ToolRegistry`](crate::ToolRegistry)). A node reads/writes whatever it
/// needs on `C` and returns the name of the next node to run, or [`NodeOutcome::Halt`].
pub trait Node<C> {
    /// The name other nodes use in their [`NodeOutcome::Goto`] to reach this node.
    fn name(&self) -> &str;

    /// Runs this node's logic against the shared context.
    fn run(&self, ctx: &mut C) -> NodeOutcome;
}

/// A named graph of [`Node`]s executed as a state machine: each node decides
/// the next node to run (or that the run should halt), rather than the graph
/// enumerating edges up front. This deliberately allows cycles (e.g. a
/// "retry the tool call" loop), guarded by a step budget so a runaway graph
/// fails loudly instead of hanging.
///
/// # Example
///
/// ```
/// use tpt_agent_graph::{AgentGraph, Node, NodeOutcome};
///
/// struct Counter { count: u32 }
///
/// struct Increment;
/// impl Node<Counter> for Increment {
///     fn name(&self) -> &str { "increment" }
///     fn run(&self, ctx: &mut Counter) -> NodeOutcome {
///         ctx.count += 1;
///         if ctx.count < 3 {
///             NodeOutcome::goto("increment")
///         } else {
///             NodeOutcome::Halt
///         }
///     }
/// }
///
/// let mut graph = AgentGraph::new("increment");
/// graph.add_node(Increment);
///
/// let mut ctx = Counter { count: 0 };
/// let path = graph.run(&mut ctx).unwrap();
/// assert_eq!(ctx.count, 3);
/// assert_eq!(path, vec!["increment", "increment", "increment"]);
/// ```
pub struct AgentGraph<C> {
    entry: String,
    nodes: BTreeMap<String, Box<dyn Node<C>>>,
    step_budget: usize,
}

/// Default cap on the number of node executions per [`AgentGraph::run`] call.
pub const DEFAULT_STEP_BUDGET: usize = 1_000;

impl<C> AgentGraph<C> {
    /// Creates a graph that starts execution at the node named `entry`.
    pub fn new(entry: impl Into<String>) -> Self {
        Self {
            entry: entry.into(),
            nodes: BTreeMap::new(),
            step_budget: DEFAULT_STEP_BUDGET,
        }
    }

    /// Overrides the default step budget (see [`GraphError::StepBudgetExceeded`]).
    pub fn with_step_budget(mut self, budget: usize) -> Self {
        self.step_budget = budget;
        self
    }

    /// Adds a node, keyed by its own [`Node::name`].
    pub fn add_node(&mut self, node: impl Node<C> + 'static) -> &mut Self {
        self.nodes.insert(node.name().to_string(), Box::new(node));
        self
    }

    /// Runs the graph starting at the entry node until a node returns
    /// [`NodeOutcome::Halt`], returning the ordered list of node names visited.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::MissingEntryNode`] if the entry node was never
    /// added, [`GraphError::UnknownNode`] if a node's outcome names a node
    /// that was never added, or [`GraphError::StepBudgetExceeded`] if
    /// execution never halts within the configured budget.
    pub fn run(&self, ctx: &mut C) -> Result<Vec<String>, GraphError> {
        if !self.nodes.contains_key(&self.entry) {
            return Err(GraphError::MissingEntryNode(self.entry.clone()));
        }

        let mut path = Vec::new();
        let mut current = self.entry.clone();

        loop {
            if path.len() >= self.step_budget {
                return Err(GraphError::StepBudgetExceeded {
                    limit: self.step_budget,
                });
            }

            let node = self
                .nodes
                .get(&current)
                .ok_or_else(|| GraphError::UnknownNode(current.clone()))?;
            path.push(current.clone());

            match node.run(ctx) {
                NodeOutcome::Halt => return Ok(path),
                NodeOutcome::Goto(next) => current = next,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct Echo(&'static str, &'static str);
    impl Node<Vec<String>> for Echo {
        fn name(&self) -> &str {
            self.0
        }
        fn run(&self, ctx: &mut Vec<String>) -> NodeOutcome {
            ctx.push(self.0.to_string());
            if self.1.is_empty() {
                NodeOutcome::Halt
            } else {
                NodeOutcome::goto(self.1)
            }
        }
    }

    #[test]
    fn runs_a_linear_chain() {
        let mut graph = AgentGraph::new("a");
        graph.add_node(Echo("a", "b"));
        graph.add_node(Echo("b", ""));

        let mut ctx = Vec::new();
        let path = graph.run(&mut ctx).unwrap();
        assert_eq!(path, vec!["a", "b"]);
        assert_eq!(ctx, vec!["a", "b"]);
    }

    #[test]
    fn missing_entry_node_is_an_error() {
        let graph: AgentGraph<Vec<String>> = AgentGraph::new("missing");
        let mut ctx = Vec::new();
        assert_eq!(
            graph.run(&mut ctx),
            Err(GraphError::MissingEntryNode("missing".to_string()))
        );
    }

    #[test]
    fn goto_unknown_node_is_an_error() {
        let mut graph = AgentGraph::new("a");
        graph.add_node(Echo("a", "nowhere"));
        let mut ctx = Vec::new();
        assert_eq!(
            graph.run(&mut ctx),
            Err(GraphError::UnknownNode("nowhere".to_string()))
        );
    }

    #[test]
    fn cycles_hit_the_step_budget() {
        let mut graph = AgentGraph::new("a").with_step_budget(5);
        graph.add_node(Echo("a", "a"));
        let mut ctx = Vec::new();
        assert_eq!(
            graph.run(&mut ctx),
            Err(GraphError::StepBudgetExceeded { limit: 5 })
        );
    }

    #[test]
    fn re_adding_a_node_replaces_it() {
        let mut graph = AgentGraph::new("a");
        graph.add_node(Echo("a", "b"));
        graph.add_node(Echo("a", ""));
        graph.add_node(Echo("b", ""));

        let mut ctx = Vec::new();
        let path = graph.run(&mut ctx).unwrap();
        assert_eq!(path, vec!["a"]);
    }
}
