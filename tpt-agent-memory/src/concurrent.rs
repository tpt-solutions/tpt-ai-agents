//! Thread-safe wrapper around [`MemoryStore`] (requires the `std` feature).

use alloc::sync::Arc;
use alloc::vec::Vec;
use std::sync::RwLock;

use crate::{MemoryEntry, MemoryStore, SearchQuery};

/// A `Send + Sync` handle to a [`MemoryStore`], safe to share and mutate
/// across threads. Cloning is cheap (it clones the underlying `Arc`), and all
/// clones observe the same underlying store.
#[derive(Clone)]
pub struct ConcurrentMemoryStore {
    inner: Arc<RwLock<MemoryStore>>,
}

impl ConcurrentMemoryStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(MemoryStore::new())),
        }
    }

    pub fn insert(&self, entry: MemoryEntry) {
        self.inner
            .write()
            .expect("ConcurrentMemoryStore lock poisoned")
            .insert(entry);
    }

    pub fn get(&self, id: &str) -> Option<MemoryEntry> {
        self.inner
            .read()
            .expect("ConcurrentMemoryStore lock poisoned")
            .get(id)
            .cloned()
    }

    pub fn search(&self, query: &SearchQuery) -> Vec<MemoryEntry> {
        self.inner
            .read()
            .expect("ConcurrentMemoryStore lock poisoned")
            .search(query)
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn len(&self) -> usize {
        self.inner
            .read()
            .expect("ConcurrentMemoryStore lock poisoned")
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner
            .read()
            .expect("ConcurrentMemoryStore lock poisoned")
            .is_empty()
    }

    pub fn clear(&self) {
        self.inner
            .write()
            .expect("ConcurrentMemoryStore lock poisoned")
            .clear();
    }
}

impl Default for ConcurrentMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_insert_and_search() {
        let store = ConcurrentMemoryStore::new();
        let mut handles = Vec::new();
        for i in 0..8 {
            let store = store.clone();
            handles.push(thread::spawn(move || {
                store.insert(MemoryEntry::with_id(
                    &alloc::format!("id_{i}"),
                    "shared content",
                    &["tag"],
                ));
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(store.len(), 8);
        let query = SearchQuery::new("shared").with_limit(100);
        assert_eq!(store.search(&query).len(), 8);
    }

    #[test]
    fn test_clone_shares_state() {
        let store = ConcurrentMemoryStore::new();
        let clone = store.clone();
        store.insert(MemoryEntry::new("hello", &[]));
        assert_eq!(clone.len(), 1);
    }

    fn _assert_send_sync<T: Send + Sync>() {}
    #[test]
    fn test_is_send_and_sync() {
        _assert_send_sync::<ConcurrentMemoryStore>();
    }
}
