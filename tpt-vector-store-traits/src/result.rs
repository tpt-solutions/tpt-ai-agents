use serde::{Deserialize, Serialize};

/// Search result from a vector store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<P> {
    pub id: alloc::string::String,
    pub score: f32,
    pub vector: Option<alloc::vec::Vec<f32>>,
    pub payload: Option<P>,
}

impl<P> SearchResult<P> {
    pub fn new(id: &str, score: f32) -> Self {
        Self {
            id: alloc::string::String::from(id),
            score,
            vector: None,
            payload: None,
        }
    }
}
