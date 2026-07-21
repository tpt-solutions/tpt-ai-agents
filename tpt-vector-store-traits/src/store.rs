use alloc::boxed::Box;
use async_trait::async_trait;

use crate::{Error, Query, SearchResult};

/// Core vector store trait.
#[async_trait]
pub trait VectorStore: Send + Sync {
    type Error: From<Error> + core::fmt::Display;
    type Distance: crate::Distance;
    type Payload: Send + Sync;

    /// Search for similar vectors.
    async fn search(&self, query: &Query) -> core::result::Result<alloc::vec::Vec<SearchResult<Self::Payload>>, Self::Error>;

    /// Insert vectors.
    async fn upsert(
        &self,
        ids: &[&str],
        vectors: &[&[f32]],
        payloads: &[Option<&Self::Payload>],
    ) -> core::result::Result<(), Self::Error>;

    /// Delete vectors by ID.
    async fn delete(&self, ids: &[&str]) -> core::result::Result<(), Self::Error>;

    /// Get collection info.
    async fn info(&self) -> core::result::Result<CollectionInfo, Self::Error>;
}

/// Collection information.
#[derive(Debug, Clone)]
pub struct CollectionInfo {
    pub name: alloc::string::String,
    pub vectors_count: usize,
    pub dimension: usize,
}
