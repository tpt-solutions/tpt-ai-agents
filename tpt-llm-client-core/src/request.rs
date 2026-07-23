use crate::response::ToolCall;
use serde::{Deserialize, Serialize};

/// Chat completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: alloc::string::String,
    pub messages: alloc::vec::Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Tool definitions for function-calling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<alloc::vec::Vec<crate::response::Tool>>,
}

/// A chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<alloc::string::String>,
    /// Tool calls requested by the model (present when `finish_reason` is
    /// `"tool_calls"` on OpenAI or when Anthropic returns `tool_use` blocks).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<alloc::vec::Vec<ToolCall>>,
    /// For `role = "tool"`: the tool-call ID this message responds to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<alloc::string::String>,
}

impl Message {
    /// Create a system message.
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a user message.
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create an assistant message.
    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a tool-result message.
    pub fn tool(tool_call_id: &str, content: &str) -> Self {
        Self {
            role: Role::Tool,
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }

    /// Returns the text content, if any.
    pub fn text(&self) -> &str {
        self.content.as_deref().unwrap_or("")
    }
}

/// Message role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    /// Tool result message (follows a tool-call from the assistant).
    Tool,
}
