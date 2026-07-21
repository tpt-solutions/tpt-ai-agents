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
//! use tpt_llm_client_core::{SseClient, ChatRequest, Message, Role};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = SseClient::openai("https://api.openai.com/v1", "sk-...");
//!     let request = ChatRequest {
//!         model: "gpt-4o-mini".into(),
//!         messages: vec![Message { role: Role::User, content: "Hello".into() }],
//!         temperature: None,
//!         max_tokens: None,
//!         stream: None,
//!     };
//!     let response = client.send(&request).await.unwrap();
//!     println!("{:?}", response);
//! }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod event;
#[cfg(feature = "std")]
mod http;
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
    base_url: alloc::string::String,
    api_key: alloc::string::String,
    provider: Provider,
    http: reqwest::Client,
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
            http: reqwest::Client::new(),
        }
    }

    pub fn anthropic(base_url: &str, api_key: &str) -> Self {
        Self {
            base_url: alloc::string::String::from(base_url),
            api_key: alloc::string::String::from(api_key),
            provider: Provider::Anthropic,
            http: reqwest::Client::new(),
        }
    }

    pub fn ollama(base_url: &str) -> Self {
        Self {
            base_url: alloc::string::String::from(base_url),
            api_key: alloc::string::String::new(),
            provider: Provider::Ollama,
            http: reqwest::Client::new(),
        }
    }

    /// Send a non-streaming chat completion request.
    pub async fn send(&self, request: &ChatRequest) -> Result<ChatResponse> {
        http::send(&self.http, &self.base_url, &self.api_key, self.provider, request).await
    }

    /// Send a streaming chat completion request, returning a stream of
    /// unified [`StreamChunk`] items as they arrive from the provider.
    pub async fn stream(
        &self,
        request: &ChatRequest,
    ) -> Result<impl futures::Stream<Item = Result<StreamChunk>>> {
        http::stream(&self.http, &self.base_url, &self.api_key, self.provider, request).await
    }
}

/// No-std compatible SSE parser for streaming responses.
pub use parser::SseParser as CoreSseParser;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
