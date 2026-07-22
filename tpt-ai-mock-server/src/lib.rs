//! Mock server for testing LLM integrations.
//!
//! A lightweight HTTP server serving pre-recorded SSE streams or plain JSON
//! responses on `POST /v1/chat/completions`, and validating incoming
//! request schemas (`model`/`messages` required, else `400`). Responses are
//! served FIFO per call to [`MockServer::add_response`], so a test can
//! script a multi-turn conversation (e.g. a tool-call turn followed by a
//! final-answer turn) against one running server.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//! - `json-sse`: Enables JSON support (enabled by default)
//!
//! # Example
//!
//! ```no_run
//! use tpt_ai_mock_server::{MockServer, RecordedResponse};
//!
//! # async fn run() {
//! let mut server = MockServer::new();
//! server.add_response(RecordedResponse::json_response(
//!     r#"{"id":"1","choices":[{"index":0,"message":{"role":"assistant","content":"hi"},"finish_reason":"stop"}]}"#,
//! ));
//! let addr = server.start().await.unwrap();
//! println!("Mock server running on {addr}");
//! # }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod response;
mod router;
mod server;
mod validation;

pub use error::Error;
pub use response::{RecordedResponse, SseChunk};
pub use server::MockServer;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
