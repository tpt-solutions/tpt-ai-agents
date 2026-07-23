use crate::error::ToolDispatchError;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

/// A single tool handler: takes the raw JSON-encoded arguments from a model's
/// tool call and returns the JSON-encoded (or plain string) result.
type Handler = Box<dyn Fn(&str) -> Result<String, String>>;

/// Maps tool names to handlers, so a model's `tool_calls[i].function.name`
/// can be routed to the right function without consumers hand-writing a
/// `match` arm per tool.
///
/// This is the piece `#[tool]` (from `tpt-tool-use-macros`) deliberately
/// leaves out: the macro generates a `<fn>_call(args_json)` function per
/// tool, but nothing to look one up *by name* when more than one tool is in
/// play. Register each generated `_call` function here instead of writing
/// the dispatch `match` by hand.
///
/// # Example
///
/// ```
/// use tpt_agent_graph::ToolRegistry;
///
/// fn get_weather_call(args_json: &str) -> Result<String, String> {
///     let _ = args_json;
///     Ok("72F and sunny".to_string())
/// }
///
/// let mut registry = ToolRegistry::new();
/// registry.register("get_weather", get_weather_call);
///
/// let result = registry.dispatch("get_weather", r#"{"location":"Austin"}"#).unwrap();
/// assert_eq!(result, "72F and sunny");
///
/// assert!(registry.dispatch("unknown_tool", "{}").is_err());
/// ```
#[derive(Default)]
pub struct ToolRegistry {
    handlers: BTreeMap<String, Handler>,
}

impl ToolRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    /// Registers a handler under `name`. A second registration for the same
    /// name replaces the first.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        handler: impl Fn(&str) -> Result<String, String> + 'static,
    ) -> &mut Self {
        self.handlers.insert(name.into(), Box::new(handler));
        self
    }

    /// Returns `true` if a handler is registered under `name`.
    pub fn contains(&self, name: &str) -> bool {
        self.handlers.contains_key(name)
    }

    /// Looks up the handler registered under `name` and invokes it with
    /// `args_json`.
    ///
    /// # Errors
    ///
    /// Returns [`ToolDispatchError::UnknownTool`] if no handler is registered
    /// under `name`, or [`ToolDispatchError::Handler`] if the handler itself
    /// returns an error.
    pub fn dispatch(&self, name: &str, args_json: &str) -> Result<String, ToolDispatchError> {
        let handler = self
            .handlers
            .get(name)
            .ok_or_else(|| ToolDispatchError::UnknownTool(name.to_string()))?;
        handler(args_json).map_err(ToolDispatchError::Handler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatches_registered_tool() {
        let mut registry = ToolRegistry::new();
        registry.register("echo", |args| Ok(args.to_string()));
        assert_eq!(registry.dispatch("echo", "hello").unwrap(), "hello");
    }

    #[test]
    fn unknown_tool_is_an_error() {
        let registry = ToolRegistry::new();
        assert_eq!(
            registry.dispatch("missing", "{}"),
            Err(ToolDispatchError::UnknownTool("missing".to_string()))
        );
    }

    #[test]
    fn handler_error_is_propagated() {
        let mut registry = ToolRegistry::new();
        registry.register("fails", |_| Err("bad args".to_string()));
        assert_eq!(
            registry.dispatch("fails", "{}"),
            Err(ToolDispatchError::Handler("bad args".to_string()))
        );
    }

    #[test]
    fn contains_reflects_registration() {
        let mut registry = ToolRegistry::new();
        assert!(!registry.contains("echo"));
        registry.register("echo", |args| Ok(args.to_string()));
        assert!(registry.contains("echo"));
    }

    #[test]
    fn re_registering_replaces_handler() {
        let mut registry = ToolRegistry::new();
        registry.register("echo", |_| Ok("first".to_string()));
        registry.register("echo", |_| Ok("second".to_string()));
        assert_eq!(registry.dispatch("echo", "{}").unwrap(), "second");
    }
}
