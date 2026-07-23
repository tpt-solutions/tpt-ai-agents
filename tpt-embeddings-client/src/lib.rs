//! Unified async client for embedding APIs (OpenAI, Cohere, custom).
//!
//! This crate provides a trait-based abstraction for generating text
//! embeddings and ready-made adapters for the most common providers.
//! Use it to feed embeddings into [`tpt_rag_pipeline`] or
//! [`tpt_agent_memory`].
//!
//! # Features
//!
//! - `std` (default): Enables networking via reqwest and the OpenAI/Cohere
//!   implementations
//! - `async`: Alias for `std`
//! - `json-sse`: No-op (reserved for future JSON response support)
//!
//! # Example
//!
//! ```no_run
//! use tpt_embeddings_client::{OpenAiEmbeddings, EmbeddingRequest, EmbeddingsClient};
//!
//! # async fn run() -> Result<(), tpt_embeddings_client::Error> {
//! let client = OpenAiEmbeddings::new("https://api.openai.com/v1", "sk-...");
//! let response = client
//!     .embed(&EmbeddingRequest {
//!         model: "text-embedding-3-small".into(),
//!         inputs: vec!["hello world".into(), "foo bar".into()],
//!         dimensions: None,
//!     })
//!     .await?;
//! assert_eq!(response.embeddings.len(), 2);
//! # Ok(())
//! # }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod traits;

pub use error::Error;
pub use traits::{
    EmbeddingMeta, EmbeddingRequest, EmbeddingResponse, EmbeddingResult, EmbeddingsClient,
};

#[cfg(feature = "std")]
mod cohere;
#[cfg(feature = "std")]
mod openai;

#[cfg(feature = "std")]
pub use cohere::CohereEmbeddings;
#[cfg(feature = "std")]
pub use openai::OpenAiEmbeddings;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_embedding_request_builder() {
        let req = EmbeddingRequest {
            model: "test-model".into(),
            inputs: vec!["hello".into(), "world".into()],
            dimensions: None,
        };
        assert_eq!(req.model, "test-model");
        assert_eq!(req.inputs.len(), 2);
    }

    #[test]
    fn test_embedding_response_meta_default() {
        let meta = EmbeddingMeta::default();
        assert!(meta.total_tokens.is_none());
        assert!(meta.model.is_none());
    }

    #[test]
    fn test_error_display() {
        let err = Error::EmptyBatch;
        assert_eq!(alloc::format!("{err}"), "input batch is empty");

        let err = Error::Provider {
            code: 401,
            message: "unauthorized".into(),
        };
        assert!(alloc::format!("{err}").contains("401"));
    }

    #[test]
    fn test_embedding_result_clone() {
        let r = EmbeddingResult {
            id: "test".into(),
            embedding: vec![0.1, 0.2, 0.3],
        };
        let r2 = r.clone();
        assert_eq!(r.id, r2.id);
        assert_eq!(r.embedding, r2.embedding);
    }
}
