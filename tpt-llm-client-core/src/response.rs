use crate::Message;
use serde::{Deserialize, Serialize};

/// Complete chat response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: alloc::string::String,
    pub choices: alloc::vec::Vec<Choice>,
    pub usage: Option<Usage>,
}

/// A single choice in the response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<alloc::string::String>,
}

/// A tool call requested by the model in a non-streaming response.
///
/// OpenAI returns these in `choice.message.tool_calls`. Anthropic returns
/// them as `tool_use` content blocks. This struct provides a unified shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Index of this tool call (for ordering).
    #[serde(default)]
    pub index: u32,
    /// Provider-assigned tool-call ID (used in the `tool` role follow-up).
    pub id: alloc::string::String,
    /// The tool type (typically `"function"`).
    #[serde(rename = "type")]
    pub tool_type: alloc::string::String,
    /// The function being called.
    pub function: ToolCallFunction,
}

/// Function details inside a [`ToolCall`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    /// Function name matching one of the tools in the request.
    pub name: alloc::string::String,
    /// JSON-encoded arguments string.
    pub arguments: alloc::string::String,
}

/// A tool definition sent in [`crate::ChatRequest::tools`].
///
/// Follows the OpenAI `tools` format:
/// ```json
/// {
///   "type": "function",
///   "function": {
///     "name": "get_weather",
///     "description": "Get weather for a location",
///     "parameters": { "type": "object", "properties": { "location": { "type": "string" } } }
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: alloc::string::String,
    pub function: ToolFunction,
}

/// Function definition inside a [`Tool`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: alloc::string::String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<alloc::string::String>,
    pub parameters: serde_json::Value,
}

/// Usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Streaming chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub id: alloc::string::String,
    pub choices: alloc::vec::Vec<StreamChoice>,
}

/// A choice in a streaming chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<alloc::string::String>,
}

/// Delta in a streaming chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<alloc::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<alloc::string::String>,
    /// Partial tool-call deltas from the streaming response. OpenAI and
    /// Anthropic stream tool-call arguments incrementally — each delta
    /// contains a `ToolCallDelta` with the argument fragment that should
    /// be appended to the corresponding tool call's argument buffer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<alloc::vec::Vec<ToolCallDelta>>,
}

/// A single tool-call delta from a streaming chunk.
///
/// OpenAI streams tool calls as:
/// ```json
/// { "index": 0, "id": "call_abc", "type": "function", "function": { "name": "get_weather", "arguments": "" } }
/// ```
/// Subsequent deltas for the same call:
/// ```json
/// { "index": 0, "function": { "arguments": "{\n \"loc" } }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Index of this tool call in the delta's `tool_calls` array.
    pub index: u32,
    /// The tool-call ID (only in the first delta for each call).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<alloc::string::String>,
    /// The tool type (only in the first delta for each call, typically `"function"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub tool_type: Option<alloc::string::String>,
    /// Function details (present in most deltas).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<ToolCallFunctionDelta>,
}

/// Function details within a tool-call delta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunctionDelta {
    /// Function name (only in the first delta for each call).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<alloc::string::String>,
    /// Partial JSON arguments (accumulated across deltas to form the full arguments string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<alloc::string::String>,
}
