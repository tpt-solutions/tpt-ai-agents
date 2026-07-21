//! Chunking, embedding batching, and context-window management.
//!
//! Provides a streaming chunker that respects token limits and handles
//! overlapping windows for RAG pipelines.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//! - `full`: Enables all integrations (tpt-tokenizers-fast, tpt-vector-store-traits, tpt-llm-client-core)
//!
//! # Example
//!
//! ```
//! use tpt_rag_pipeline::{Chunker, ChunkConfig};
//!
//! let config = ChunkConfig::new(10, 2);
//! let chunker = Chunker::new(config);
//! let chunks = chunker.chunk("one two three four five six seven eight nine ten").unwrap();
//! assert!(chunks.len() > 1);
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod chunker;
mod config;
mod context;
mod embedding;
mod error;

pub use chunker::{Chunk, Chunker};
pub use config::ChunkConfig;
pub use context::ContextWindow;
pub use embedding::EmbeddingBatch;
pub use error::Error;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
