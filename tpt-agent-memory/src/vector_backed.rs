//! Optional integration with a real [`VectorStore`] backend (e.g. Qdrant,
//! pgvector — see `tpt-vector-store-traits`) for embedding search, in place
//! of [`MemoryStore::search`]'s in-memory cosine-similarity scan. Requires
//! the `vector-store` feature.

use alloc::vec::Vec;
use tpt_vector_store_traits::{Query, VectorStore};

use crate::{Error, MemoryEntry, MemoryStore};

/// Wraps a [`MemoryStore`] (source of truth for entry content, and for
/// keyword search) with a real [`VectorStore`] backend used purely as an ANN
/// index: entry IDs and embeddings are upserted into it, and semantic search
/// queries it directly and resolves the returned IDs back to full entries
/// via the local store. The backend never receives entry content — only IDs
/// and vectors — so this works with any `VectorStore` implementation
/// regardless of its `Payload` type.
pub struct VectorBackedMemoryStore<S: VectorStore> {
    local: MemoryStore,
    backend: S,
}

impl<S: VectorStore> VectorBackedMemoryStore<S> {
    pub fn new(backend: S) -> Self {
        Self {
            local: MemoryStore::new(),
            backend,
        }
    }

    /// Insert an entry into the local store, and — if it carries an
    /// embedding — upsert that embedding into the vector store backend
    /// keyed by the entry's ID.
    pub async fn insert(&mut self, entry: MemoryEntry) -> crate::Result<()> {
        if let Some(embedding) = &entry.embedding {
            self.backend
                .upsert(&[&entry.id], &[embedding.as_slice()], &[None])
                .await
                .map_err(|e| Error::Storage(alloc::format!("vector store upsert failed: {e}")))?;
        }
        self.local.insert(entry);
        Ok(())
    }

    /// Remove an entry from both the local store and the vector backend.
    pub async fn remove(&mut self, id: &str) -> crate::Result<()> {
        self.backend
            .delete(&[id])
            .await
            .map_err(|e| Error::Storage(alloc::format!("vector store delete failed: {e}")))?;
        self.local.remove(id);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&MemoryEntry> {
        self.local.get(id)
    }

    /// Keyword (substring/tag) search — entirely local, does not touch the
    /// vector store backend. See [`MemoryStore::search`].
    pub fn search(&self, query: &crate::SearchQuery) -> Vec<&MemoryEntry> {
        self.local.search(query)
    }

    /// Semantic search via the vector store backend: queries it for the
    /// nearest `limit` vectors to `query_embedding`, then resolves each
    /// returned ID back to its full entry in the local store (skipping any
    /// ID the backend returns that isn't known locally — e.g. if the two
    /// stores have drifted out of sync).
    pub async fn search_semantic(
        &self,
        query_embedding: Vec<f32>,
        limit: usize,
    ) -> crate::Result<Vec<&MemoryEntry>> {
        let query = Query {
            vector: query_embedding,
            filter: None,
            limit,
            offset: None,
            include_payload: Some(false),
        };
        let results = self
            .backend
            .search(&query)
            .await
            .map_err(|e| Error::Storage(alloc::format!("vector store search failed: {e}")))?;

        Ok(results
            .into_iter()
            .filter_map(|r| self.local.get(&r.id))
            .collect())
    }

    pub fn len(&self) -> usize {
        self.local.len()
    }

    pub fn is_empty(&self) -> bool {
        self.local.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SearchQuery;
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::sync::Arc;
    use async_trait::async_trait;
    use std::sync::Mutex;
    use tpt_vector_store_traits::{CollectionInfo, Error as VsError, SearchResult};

    type StoredVectors = alloc::vec::Vec<(String, Vec<f32>)>;

    /// An in-memory `VectorStore` fake — enough to prove the wiring above
    /// actually reaches the backend, without requiring a real Qdrant/pgvector
    /// deployment in tests.
    #[derive(Clone, Default)]
    struct FakeVectorStore {
        vectors: Arc<Mutex<StoredVectors>>,
    }

    fn cosine(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if na == 0.0 || nb == 0.0 {
            0.0
        } else {
            dot / (na * nb)
        }
    }

    #[async_trait]
    impl VectorStore for FakeVectorStore {
        type Error = VsError;
        type Distance = tpt_vector_store_traits::CosineDistance;
        type Payload = ();

        async fn search(&self, query: &Query) -> Result<Vec<SearchResult<()>>, Self::Error> {
            let vectors = self.vectors.lock().unwrap();
            let mut scored: Vec<(String, f32)> = vectors
                .iter()
                .map(|(id, v)| (id.clone(), cosine(&query.vector, v)))
                .collect();
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            scored.truncate(query.limit);
            Ok(scored
                .into_iter()
                .map(|(id, score)| SearchResult::new(&id, score))
                .collect())
        }

        async fn upsert(
            &self,
            ids: &[&str],
            vecs: &[&[f32]],
            _payloads: &[Option<&()>],
        ) -> Result<(), Self::Error> {
            let mut vectors = self.vectors.lock().unwrap();
            for (id, v) in ids.iter().zip(vecs) {
                vectors.push((String::from(*id), v.to_vec()));
            }
            Ok(())
        }

        async fn delete(&self, ids: &[&str]) -> Result<(), Self::Error> {
            let mut vectors = self.vectors.lock().unwrap();
            vectors.retain(|(id, _)| !ids.contains(&id.as_str()));
            Ok(())
        }

        async fn info(&self) -> Result<CollectionInfo, Self::Error> {
            Ok(CollectionInfo {
                name: String::from("fake"),
                vectors_count: self.vectors.lock().unwrap().len(),
                dimension: 0,
            })
        }
    }

    #[tokio::test]
    async fn test_insert_upserts_embedding_into_backend() {
        let mut store = VectorBackedMemoryStore::new(FakeVectorStore::default());
        store
            .insert(
                MemoryEntry::with_id("a", "close match", &[]).with_embedding(alloc::vec![1.0, 0.0]),
            )
            .await
            .unwrap();
        store
            .insert(
                MemoryEntry::with_id("b", "far match", &[]).with_embedding(alloc::vec![0.0, 1.0]),
            )
            .await
            .unwrap();

        let results = store
            .search_semantic(alloc::vec![1.0, 0.0], 10)
            .await
            .unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "a");
        assert_eq!(results[1].id, "b");
    }

    #[tokio::test]
    async fn test_keyword_search_stays_local() {
        let mut store = VectorBackedMemoryStore::new(FakeVectorStore::default());
        store
            .insert(MemoryEntry::new("dark mode preference", &["pref"]))
            .await
            .unwrap();

        let results = store.search(&SearchQuery::new("dark"));
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_remove_deletes_from_both() {
        let mut store = VectorBackedMemoryStore::new(FakeVectorStore::default());
        store
            .insert(MemoryEntry::with_id("a", "hi", &[]).with_embedding(alloc::vec![1.0, 0.0]))
            .await
            .unwrap();
        store.remove("a").await.unwrap();

        assert!(store.get("a").is_none());
        let results = store
            .search_semantic(alloc::vec![1.0, 0.0], 10)
            .await
            .unwrap();
        assert!(results.is_empty());
    }
}
