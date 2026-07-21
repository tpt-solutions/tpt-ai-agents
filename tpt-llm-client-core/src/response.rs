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
}
