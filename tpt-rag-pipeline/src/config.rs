/// Configuration for the chunker.
#[derive(Debug, Clone)]
pub struct ChunkConfig {
    pub max_tokens: usize,
    pub overlap: usize,
}

impl ChunkConfig {
    pub fn new(max_tokens: usize, overlap: usize) -> Self {
        Self {
            max_tokens,
            overlap,
        }
    }
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            overlap: 50,
        }
    }
}
