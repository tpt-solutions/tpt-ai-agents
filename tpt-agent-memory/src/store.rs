use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::search::cosine_similarity;
#[cfg(feature = "std")]
use crate::Error;
use crate::{MemoryEntry, SearchQuery};

/// In-memory key-value store for memory entries.
///
/// Not thread-safe by itself (plain `BTreeMap` behind `&mut self`). For
/// concurrent access from multiple threads, use [`crate::ConcurrentMemoryStore`]
/// (requires the `std` feature), which wraps this type in `Arc<RwLock<..>>`.
///
/// With the `std` feature, entries can be persisted to and restored from a
/// JSON file via [`MemoryStore::save_to_file`] / [`MemoryStore::load_from_file`].
/// This is file-based persistence only — there is no database backend.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemoryStore {
    entries: BTreeMap<String, MemoryEntry>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, entry: MemoryEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    pub fn get(&self, id: &str) -> Option<&MemoryEntry> {
        self.entries.get(id)
    }

    /// Search entries by substring/tag match, or by cosine-similarity
    /// against `query.embedding` when set (see
    /// [`SearchQuery::with_embedding`]).
    ///
    /// Substring search ranks by `score` (highest first); embedding search
    /// ranks by similarity (highest first) and only considers entries that
    /// carry an [`MemoryEntry::embedding`].
    pub fn search(&self, query: &SearchQuery) -> Vec<&MemoryEntry> {
        if let Some(query_embedding) = &query.embedding {
            let mut results: Vec<(&MemoryEntry, f32)> = self
                .entries
                .values()
                .filter(|e| e.score >= query.min_score)
                .filter_map(|e| {
                    e.embedding
                        .as_ref()
                        .map(|emb| (e, cosine_similarity(query_embedding, emb)))
                })
                .collect();

            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
            results.truncate(query.limit);
            return results.into_iter().map(|(e, _)| e).collect();
        }

        let mut results: Vec<&MemoryEntry> = self
            .entries
            .values()
            .filter(|e| {
                e.content.contains(&query.text) || e.tags.iter().any(|t| t.contains(&query.text))
            })
            .filter(|e| e.score >= query.min_score)
            .collect();

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(core::cmp::Ordering::Equal)
        });
        results.truncate(query.limit);
        results
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Serialize all entries as JSON and write them to `path`, overwriting
    /// any existing file. Requires the `std` feature.
    #[cfg(feature = "std")]
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> crate::Result<()> {
        let json = serde_json::to_string(self)
            .map_err(|e| Error::Serialization(alloc::format!("{e}")))?;
        std::fs::write(path, json).map_err(|e| Error::Storage(alloc::format!("{e}")))
    }

    /// Load a store previously written by [`MemoryStore::save_to_file`].
    /// Requires the `std` feature.
    #[cfg(feature = "std")]
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        let json =
            std::fs::read_to_string(path).map_err(|e| Error::Storage(alloc::format!("{e}")))?;
        serde_json::from_str(&json).map_err(|e| Error::Serialization(alloc::format!("{e}")))
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut store = MemoryStore::new();
        store.insert(MemoryEntry::new("dark mode preference", &["pref"]));
        store.insert(MemoryEntry::new("light theme", &["theme"]));

        let query = SearchQuery::new("dark");
        let results = store.search(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "dark mode preference");
    }

    #[test]
    fn test_embedding_search_ranks_by_similarity() {
        let mut store = MemoryStore::new();
        store.insert(
            MemoryEntry::with_id("close", "close match", &[]).with_embedding(alloc::vec![
                1.0, 0.0, 0.0
            ]),
        );
        store.insert(
            MemoryEntry::with_id("far", "far match", &[]).with_embedding(alloc::vec![
                0.0, 1.0, 0.0
            ]),
        );
        store.insert(MemoryEntry::with_id("no_embedding", "no embedding", &[]));

        let query = SearchQuery::new("").with_embedding(alloc::vec![1.0, 0.0, 0.0]);
        let results = store.search(&query);

        assert_eq!(results.len(), 2, "entry without an embedding is excluded");
        assert_eq!(results[0].id, "close");
        assert_eq!(results[1].id, "far");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_save_and_load_round_trip() {
        let mut store = MemoryStore::new();
        store.insert(MemoryEntry::with_id("a", "hello world", &["greeting"]));
        store.insert(
            MemoryEntry::with_id("b", "vector entry", &[]).with_embedding(alloc::vec![1.0, 2.0]),
        );

        let path = std::env::temp_dir().join(alloc::format!(
            "tpt_agent_memory_test_{}.json",
            std::process::id()
        ));
        store.save_to_file(&path).unwrap();
        let loaded = MemoryStore::load_from_file(&path).unwrap();
        std::fs::remove_file(&path).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.get("a").unwrap().content, "hello world");
        assert_eq!(loaded.get("b").unwrap().embedding, Some(alloc::vec![1.0, 2.0]));
    }
}
