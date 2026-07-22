use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::{MemoryEntry, SearchQuery};

/// In-memory key-value store for memory entries.
///
/// Not thread-safe by itself (plain `BTreeMap` behind `&mut self`). For
/// concurrent access from multiple threads, use [`crate::ConcurrentMemoryStore`]
/// (requires the `std` feature), which wraps this type in `Arc<RwLock<..>>`.
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

    pub fn search(&self, query: &SearchQuery) -> Vec<&MemoryEntry> {
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
}
