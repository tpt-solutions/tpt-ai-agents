use crate::{ChunkConfig, Error};

/// A chunk of text with metadata.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub text: alloc::string::String,
    pub start: usize,
    pub end: usize,
    pub token_count: usize,
}

/// Text chunker that respects token limits.
pub struct Chunker {
    config: ChunkConfig,
}

impl Chunker {
    pub fn new(config: ChunkConfig) -> Self {
        Self { config }
    }

    pub fn chunk(&self, text: &str) -> core::result::Result<alloc::vec::Vec<Chunk>, Error> {
        let mut chunks = alloc::vec::Vec::new();
        let words: alloc::vec::Vec<&str> = text.split_whitespace().collect();

        if words.is_empty() {
            return Ok(chunks);
        }

        let mut start = 0;
        while start < words.len() {
            let end = core::cmp::min(start + self.config.max_tokens, words.len());
            let chunk_text = words[start..end].join(" ");
            let token_count = chunk_text.split_whitespace().count();

            chunks.push(Chunk {
                text: chunk_text,
                start,
                end,
                token_count,
            });

            let step = self.config.max_tokens.saturating_sub(self.config.overlap);
            start += core::cmp::max(step, 1);
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_chunking() {
        let config = ChunkConfig::new(3, 0);
        let chunker = Chunker::new(config);
        let chunks = chunker.chunk("one two three four five").unwrap();
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].text, "one two three");
        assert_eq!(chunks[1].text, "four five");
    }

    #[test]
    fn test_overlapping_chunks() {
        let config = ChunkConfig::new(3, 1);
        let chunker = Chunker::new(config);
        let chunks = chunker.chunk("one two three four five").unwrap();
        assert_eq!(chunks.len(), 3);
    }
}
