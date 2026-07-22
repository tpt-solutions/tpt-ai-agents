use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// Global counter for generating unique entry IDs.
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// A single memory entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub timestamp: u64,
    pub access_count: u64,
    pub score: f32,
    /// Optional embedding vector for cosine-similarity semantic search.
    ///
    /// This crate does not compute embeddings itself — callers obtain them
    /// from an embedding model (e.g. via `tpt-llm-client-core`) and attach
    /// them here. Entries without an embedding still participate in
    /// substring/tag search via [`crate::MemoryStore::search`].
    pub embedding: Option<Vec<f32>>,
}

impl MemoryEntry {
    /// Create a new memory entry with an auto-generated unique ID.
    pub fn new(content: &str, tags: &[&str]) -> Self {
        Self {
            id: alloc::format!("mem_{}", next_id()),
            content: String::from(content),
            tags: tags.iter().map(|t| String::from(*t)).collect(),
            timestamp: 0,
            access_count: 0,
            score: 1.0,
            embedding: None,
        }
    }

    /// Create a new memory entry with a caller-specified ID.
    pub fn with_id(id: &str, content: &str, tags: &[&str]) -> Self {
        Self {
            id: String::from(id),
            content: String::from(content),
            tags: tags.iter().map(|t| String::from(*t)).collect(),
            timestamp: 0,
            access_count: 0,
            score: 1.0,
            embedding: None,
        }
    }

    /// Attach an embedding vector, enabling cosine-similarity search for
    /// this entry when queries are built with
    /// [`crate::SearchQuery::with_embedding`].
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_ids() {
        let a = MemoryEntry::new("hello", &[]);
        let b = MemoryEntry::new("hello", &[]);
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn test_with_id() {
        let entry = MemoryEntry::with_id("custom_id", "content", &["tag"]);
        assert_eq!(entry.id, "custom_id");
    }
}
