//! Standardized async traits for Qdrant, Milvus, pgvector.
//!
//! This crate defines trait abstractions for vector database operations
//! using GATs for distance metrics and payload types without boxing.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//! - `json-sse`: Enables JSON payload support (enabled by default)
//!
//! # Example
//!
//! ```rust,ignore
//! use tpt_vector_store_traits::{VectorStore, Query, SearchResult};
//!
//! struct QdrantStore { /* ... */ }
//!
//! impl VectorStore for QdrantStore {
//!     type Error = QdrantError;
//!     type Distance = CosineDistance;
//!     type Payload = serde_json::Value;
//!
//!     async fn search(&self, query: &Query) -> Result<Vec<SearchResult<Self::Payload>>, Self::Error> {
//!         todo!()
//!     }
//! }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod query;
mod result;
mod store;
mod traits;

pub use error::Error;
pub use query::{Filter, Query};
pub use result::SearchResult;
pub use store::VectorStore;

/// Distance metric trait for vector similarity.
pub trait Distance {
    fn name(&self) -> &str;
}

/// Cosine distance metric.
#[derive(Debug, Clone, Copy)]
pub struct CosineDistance;

impl Distance for CosineDistance {
    fn name(&self) -> &str {
        "cosine"
    }
}

/// Euclidean distance metric.
#[derive(Debug, Clone, Copy)]
pub struct EuclideanDistance;

impl Distance for EuclideanDistance {
    fn name(&self) -> &str {
        "euclid"
    }
}

/// Dot product distance metric.
#[derive(Debug, Clone, Copy)]
pub struct DotProductDistance;

impl Distance for DotProductDistance {
    fn name(&self) -> &str {
        "dot"
    }
}

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
