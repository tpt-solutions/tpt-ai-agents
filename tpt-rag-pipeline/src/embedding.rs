/// Batch of embeddings for vector storage.
#[derive(Debug, Clone)]
pub struct EmbeddingBatch {
    pub ids: alloc::vec::Vec<alloc::string::String>,
    pub vectors: alloc::vec::Vec<alloc::vec::Vec<f32>>,
    pub texts: alloc::vec::Vec<alloc::string::String>,
}

impl EmbeddingBatch {
    pub fn new() -> Self {
        Self {
            ids: alloc::vec::Vec::new(),
            vectors: alloc::vec::Vec::new(),
            texts: alloc::vec::Vec::new(),
        }
    }

    pub fn push(&mut self, id: &str, vector: alloc::vec::Vec<f32>, text: &str) {
        self.ids.push(alloc::string::String::from(id));
        self.vectors.push(vector);
        self.texts.push(alloc::string::String::from(text));
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

impl Default for EmbeddingBatch {
    fn default() -> Self {
        Self::new()
    }
}
