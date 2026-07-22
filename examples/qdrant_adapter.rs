//! Reference adapter example for `tpt-vector-store-traits`.
//!
//! Shows how to implement the [`VectorStore`] trait for a hypothetical
//! Qdrant client — every method body is a placeholder (`Err(...)`) since
//! this example intentionally has no dependency on a real `qdrant-client`
//! crate. A type implementing `VectorStore` like this one is exactly what
//! `tpt-agent-memory::VectorBackedMemoryStore` (behind its `vector-store`
//! feature) expects, for embedding search backed by a real vector database
//! instead of an in-memory scan.
//!
//! To use this as a starting point: add `qdrant-client` to your
//! `Cargo.toml`, copy this file into your project, and fill in each
//! method body per its comment.

use async_trait::async_trait;
use tpt_vector_store_traits::{
    CollectionInfo, CosineDistance, Error, Query, SearchResult, VectorStore,
};

/// A hypothetical Qdrant client wrapper.
///
/// In a real implementation, this would hold a `qdrant_client::Qdrant`
/// instance and map the trait methods to Qdrant's gRPC/REST API.
struct QdrantAdapter {
    _collection: String,
}

impl QdrantAdapter {
    fn new(collection: &str) -> Self {
        Self {
            _collection: collection.to_string(),
        }
    }
}

/// A provider-specific error type. The `VectorStore` trait requires
/// `Error: From<tpt_vector_store_traits::Error>`.
#[derive(Debug)]
struct QdrantError(String);

impl core::fmt::Display for QdrantError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "qdrant: {}", self.0)
    }
}

impl From<Error> for QdrantError {
    fn from(e: Error) -> Self {
        QdrantError(e.to_string())
    }
}

#[async_trait]
impl VectorStore for QdrantAdapter {
    type Error = QdrantError;
    type Distance = CosineDistance;
    type Payload = serde_json::Value;

    async fn search(&self, query: &Query) -> Result<Vec<SearchResult<Self::Payload>>, Self::Error> {
        // In a real implementation:
        // 1. Build a Qdrant SearchRequest from `query`
        // 2. Call self.client.search(request).await
        // 3. Map Qdrant's ScoredPoint to SearchResult<serde_json::Value>
        let _ = query;
        Err(QdrantError("not yet implemented — see comments".into()))
    }

    async fn upsert(
        &self,
        ids: &[&str],
        vectors: &[&[f32]],
        payloads: &[Option<&Self::Payload>],
    ) -> Result<(), Self::Error> {
        // In a real implementation:
        // 1. Build PointStructs from ids, vectors, payloads
        // 2. Call self.client.upsert_points(collection, points, None).await
        let _ = (ids, vectors, payloads);
        Err(QdrantError("not yet implemented — see comments".into()))
    }

    async fn delete(&self, ids: &[&str]) -> Result<(), Self::Error> {
        // In a real implementation:
        // self.client.delete_points(collection, ids.to_vec(), None).await
        let _ = ids;
        Err(QdrantError("not yet implemented — see comments".into()))
    }

    async fn info(&self) -> Result<CollectionInfo, Self::Error> {
        // In a real implementation:
        // self.client.collection_info(&self.collection).await
        Err(QdrantError("not yet implemented — see comments".into()))
    }
}

fn main() {
    println!("This is a reference adapter example — see the source code.");
    println!("It demonstrates how to implement VectorStore for a real backend.");
    let adapter = QdrantAdapter::new("my_collection");
    println!("Adapter created for collection: {}", adapter._collection);
}
