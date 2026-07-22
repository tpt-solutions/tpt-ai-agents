//! Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama.
//!
//! This crate provides a robust SSE parser that handles partial JSON chunks,
//! and retries requests automatically on transient failures (connection
//! errors, HTTP 429, HTTP 5xx) with exponential backoff — see [`RetryConfig`]
//! and [`SseClient::with_retry`].
//!
//! # Features
//!
//! - `std` (default): Enables networking via reqwest
//! - `async`: Alias for `std`
//! - `json-sse`: Enables SSE parsing (enabled by default)
//!
//! # Example
//!
//! ```no_run
//! use tpt_llm_client_core::{SseClient, ChatRequest, Message, Role};
//!
//! # async fn run() {
//! let client = SseClient::openai("https://api.openai.com/v1", "sk-...");
//! let request = ChatRequest {
//!     model: "gpt-4o-mini".into(),
//!     messages: vec![Message { role: Role::User, content: "Hello".into() }],
//!     temperature: None,
//!     max_tokens: None,
//!     stream: None,
//! };
//! let response = client.send(&request).await.unwrap();
//! println!("{:?}", response);
//! # }
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
    retry: RetryConfig,
}

/// Retry policy for transient request failures (connection errors, HTTP
/// 429, HTTP 5xx). Non-retryable errors (4xx other than 429, JSON/parse
/// errors) are always returned immediately regardless of this config.
#[derive(Debug, Clone, Copy)]
pub struct RetryConfig {
    /// Total attempts, including the first. `1` disables retrying.
    pub max_attempts: u32,
    /// Delay before the first retry; doubles after each subsequent attempt.
    pub base_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 200,
        }
    }
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
            retry: RetryConfig::default(),
        }
    }

    pub fn anthropic(base_url: &str, api_key: &str) -> Self {
        Self {
            base_url: alloc::string::String::from(base_url),
            api_key: alloc::string::String::from(api_key),
            provider: Provider::Anthropic,
            http: reqwest::Client::new(),
            retry: RetryConfig::default(),
        }
    }

    pub fn ollama(base_url: &str) -> Self {
        Self {
            base_url: alloc::string::String::from(base_url),
            api_key: alloc::string::String::new(),
            provider: Provider::Ollama,
            http: reqwest::Client::new(),
            retry: RetryConfig::default(),
        }
    }

    /// Override the default retry policy (3 attempts, 200ms base backoff).
    pub fn with_retry(mut self, retry: RetryConfig) -> Self {
        self.retry = retry;
        self
    }

    /// Send a non-streaming chat completion request. Transient failures are
    /// retried per the client's [`RetryConfig`].
    pub async fn send(&self, request: &ChatRequest) -> Result<ChatResponse> {
        http::send(
            &self.http,
            &self.base_url,
            &self.api_key,
            self.provider,
            request,
            &self.retry,
        )
        .await
    }

    /// Send a streaming chat completion request, returning a stream of
    /// unified [`StreamChunk`] items as they arrive from the provider.
    /// Establishing the stream is retried per the client's [`RetryConfig`];
    /// once streaming begins, a mid-stream failure is yielded as an `Err`
    /// item rather than retried.
    pub async fn stream(
        &self,
        request: &ChatRequest,
    ) -> Result<impl futures::Stream<Item = Result<StreamChunk>>> {
        http::stream(
            &self.http,
            &self.base_url,
            &self.api_key,
            self.provider,
            request,
            &self.retry,
        )
        .await
    }
}

/// No-std compatible SSE parser for streaming responses.
pub use parser::SseParser as CoreSseParser;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
