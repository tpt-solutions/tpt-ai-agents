//! Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama.
//!
//! This crate provides a robust SSE parser that handles partial JSON chunks
//! and network retries automatically.
//!
//! # Features
//!
//! - `std` (default): Enables networking via reqwest
//! - `async`: Alias for `std`
//! - `json-sse`: Enables SSE parsing (enabled by default)
//! - `tool-use`: Enables tool-use integration via `tpt-prompt-template`
//!
//! # Example
//!
//! ```rust,ignore
//! use tpt_llm_client_core::SseClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SseClient::openai("https://api.openai.com/v1", "sk-...");
//!     let mut stream = client.chat_completion("Hello").await.unwrap();
//!     while let Some(chunk) = stream.next().await {
//!         println!("{:?}", chunk);
//!     }
//! }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod event;
mod parser;
mod request;
mod response;

pub use error::Error;
pub use event::SseEvent;
pub use parser::SseParser;
pub use request::{ChatRequest, Message, Role};
pub use response::{ChatResponse, StreamChunk};

/// SSE streaming client for LLM APIs.
#[cfg(feature = "std")]
pub struct SseClient {
    #[allow(dead_code)]
    base_url: alloc::string::String,
    #[allow(dead_code)]
    api_key: alloc::string::String,
    #[allow(dead_code)]
    provider: Provider,
}

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    OpenAi,
    Anthropic,
    Ollama,
}

#[cfg(feature = "std")]
impl SseClient {
    pub fn openai(base_url: &str, api_key: &str) -> Self {
        Self {
            base_url: alloc::string::String::from(base_url),
            api_key: alloc::string::String::from(api_key),
            provider: Provider::OpenAi,
        }
    }

    pub fn anthropic(base_url: &str, api_key: &str) -> Self {
        Self {
            base_url: alloc::string::String::from(base_url),
            api_key: alloc::string::String::from(api_key),
            provider: Provider::Anthropic,
        }
    }

    pub fn ollama(base_url: &str) -> Self {
        Self {
            base_url: alloc::string::String::from(base_url),
            api_key: alloc::string::String::new(),
            provider: Provider::Ollama,
        }
    }
}

/// No-std compatible SSE parser for streaming responses.
pub use parser::SseParser as CoreSseParser;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
