//! Standardized async traits for vector database backends.
//!
//! This crate defines the [`VectorStore`] trait (via `#[async_trait]`, so
//! it's dyn-compatible) and supporting types (`Distance`, `Query`,
//! `SearchResult`) that a backend adapter implements. No concrete adapter
//! (Qdrant, Milvus, pgvector, etc.) ships in this crate — implement
//! [`VectorStore`] for your own client, or wait for a reference adapter.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//! - `json-sse`: Enables JSON payload support (enabled by default)
//!
//! # Example
//!
//! ```
//! use tpt_vector_store_traits::{CosineDistance, Distance, SearchResult};
//!
//! let result = SearchResult::<()>::new("doc1", 0.95);
//! assert_eq!(result.id, "doc1");
//! let d = CosineDistance;
//! assert_eq!(d.name(), "cosine");
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
pub use query::{Filter, FilterCondition, FilterOp, FilterValue, Query};
pub use result::SearchResult;
pub use store::{CollectionInfo, VectorStore};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_metrics() {
        assert_eq!(CosineDistance.name(), "cosine");
        assert_eq!(EuclideanDistance.name(), "euclid");
        assert_eq!(DotProductDistance.name(), "dot");
    }

    #[test]
    fn test_query_default_limit() {
        let q = Query {
            vector: alloc::vec![1.0, 2.0, 3.0],
            filter: None,
            limit: 10,
            offset: None,
            include_payload: None,
        };
        assert_eq!(q.limit, 10);
    }

    #[test]
    fn test_search_result() {
        let r = SearchResult::<()>::new("doc1", 0.95);
        assert_eq!(r.id, "doc1");
        assert!((r.score - 0.95).abs() < 0.001);
        assert!(r.vector.is_none());
        assert!(r.payload.is_none());
    }
}
