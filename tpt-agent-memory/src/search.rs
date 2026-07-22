/// Search query for memory retrieval.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub text: alloc::string::String,
    pub limit: usize,
    pub min_score: f32,
    pub tags: alloc::vec::Vec<alloc::string::String>,
    /// When set, [`crate::MemoryStore::search`] ranks entries by cosine
    /// similarity between this vector and each entry's
    /// [`crate::MemoryEntry::embedding`] instead of substring matching.
    /// Entries with no embedding are excluded from embedding-based search.
    pub embedding: Option<alloc::vec::Vec<f32>>,
}

impl SearchQuery {
    pub fn new(text: &str) -> Self {
        Self {
            text: alloc::string::String::from(text),
            limit: 10,
            min_score: 0.0,
            tags: alloc::vec::Vec::new(),
            embedding: None,
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

    /// Enable cosine-similarity semantic search using the given query
    /// embedding, ranking entries that carry an
    /// [`crate::MemoryEntry::embedding`] by similarity instead of substring
    /// matching.
    pub fn with_embedding(mut self, embedding: alloc::vec::Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }
}

/// Cosine similarity between two vectors, in `[-1.0, 1.0]`.
///
/// Returns `0.0` if either vector is empty or zero-length (dot product
/// undefined), which sorts such entries to the bottom rather than panicking
/// or dividing by zero.
pub(crate) fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a = sqrtf(a.iter().map(|x| x * x).sum::<f32>());
    let norm_b = sqrtf(b.iter().map(|x| x * x).sum::<f32>());
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

/// `f32::sqrt`, available without the `std` feature.
///
/// `core` has no floating-point `sqrt` (it requires a libm), so on `no_std`
/// this falls back to a small fixed-iteration Newton-Raphson approximation,
/// which is precise enough for similarity ranking.
#[cfg(feature = "std")]
fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}

#[cfg(not(feature = "std"))]
fn sqrtf(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut guess = x;
    for _ in 0..20 {
        guess = 0.5 * (guess + x / guess);
    }
    guess
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_cosine_similarity_identical() {
        let v = vec![1.0, 2.0, 3.0];
        assert!((cosine_similarity(&v, &v) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        assert!((cosine_similarity(&[1.0, 0.0], &[0.0, 1.0])).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_empty() {
        assert_eq!(cosine_similarity(&[], &[1.0]), 0.0);
    }
}
