/// Search query for memory retrieval.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub text: alloc::string::String,
    pub limit: usize,
    pub min_score: f32,
    pub tags: alloc::vec::Vec<alloc::string::String>,
}

impl SearchQuery {
    pub fn new(text: &str) -> Self {
        Self {
            text: alloc::string::String::from(text),
            limit: 10,
            min_score: 0.0,
            tags: alloc::vec::Vec::new(),
        }
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_min_score(mut self, min_score: f32) -> Self {
        self.min_score = min_score;
        self
    }
}
