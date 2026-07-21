//! Mock server for testing LLM integrations.
//!
//! A lightweight HTTP server serving pre-recorded SSE streams and
//! validating incoming request schemas.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//! - `json-sse`: Enables JSON support (enabled by default)
//!
//! # Example
//!
//! ```rust,ignore
//! use tpt_ai_mock_server::{MockServer, RecordedResponse};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut server = MockServer::new();
//!     server.add_response(RecordedResponse::sse_stream("Hello, world!"));
//!     let addr = server.start().await;
//!     println!("Mock server running on {}", addr);
//! }
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
