use alloc::boxed::Box;
use alloc::vec::Vec;
use async_trait::async_trait;

use crate::Error;

/// Metadata returned alongside embeddings (token usage, model info).
#[derive(Debug, Clone, Default)]
pub struct EmbeddingMeta {
    /// Total tokens consumed by the request.
    pub total_tokens: Option<u32>,
    /// Model identifier used by the provider.
    pub model: Option<alloc::string::String>,
}

/// A single embedding result with its ID, vector, and optional metadata.
#[derive(Debug, Clone)]
pub struct EmbeddingResult {
    /// The caller-supplied ID that was passed in [`EmbeddingRequest::inputs`].
    pub id: alloc::string::String,
    /// The embedding vector.
    pub embedding: Vec<f32>,
}

/// Input for the [`EmbeddingsClient::embed`] method.
#[derive(Debug, Clone)]
pub struct EmbeddingRequest {
    /// The model to use for embedding (e.g. `"text-embedding-3-small"`).
    pub model: alloc::string::String,
    /// Text inputs to embed. Each string becomes one [`EmbeddingResult`].
    pub inputs: Vec<alloc::string::String>,
    /// Optional dimensions parameter (OpenAI supports truncation via
    /// `dimensions` for some models).
    pub dimensions: Option<u32>,
}

/// Response from [`EmbeddingsClient::embed`].
#[derive(Debug, Clone)]
pub struct EmbeddingResponse {
    /// The embedding results, one per input, in the same order.
    pub embeddings: Vec<EmbeddingResult>,
    /// Provider-reported metadata.
    pub meta: EmbeddingMeta,
}

/// Unified async trait for embedding providers.
///
/// Implement this trait to plug any embedding backend into the pipeline.
/// The `std` feature provides ready-made implementations for OpenAI and
/// Cohere.
#[async_trait]
pub trait EmbeddingsClient: Send + Sync {
    /// Generate embeddings for the given batch of inputs.
    async fn embed(&self, request: &EmbeddingRequest) -> Result<EmbeddingResponse, Error>;
}
