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

    #[test]
    fn test_empty_input_produces_no_chunks() {
        let chunker = Chunker::new(ChunkConfig::new(3, 0));
        let chunks = chunker.chunk("").unwrap();
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_whitespace_only_input_produces_no_chunks() {
        let chunker = Chunker::new(ChunkConfig::new(3, 0));
        let chunks = chunker.chunk("   \n\t  ").unwrap();
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_input_smaller_than_max_tokens_yields_single_chunk() {
        let chunker = Chunker::new(ChunkConfig::new(50, 5));
        let chunks = chunker.chunk("just three words").unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "just three words");
    }

    #[test]
    fn test_unicode_words_are_not_split_mid_codepoint() {
        let chunker = Chunker::new(ChunkConfig::new(2, 0));
        let chunks = chunker.chunk("héllo wörld 日本語 emoji😀here").unwrap();
        let rejoined: alloc::string::String = chunks
            .iter()
            .flat_map(|c| c.text.split_whitespace())
            .collect::<alloc::vec::Vec<_>>()
            .join(" ");
        assert_eq!(rejoined, "héllo wörld 日本語 emoji😀here");
    }

    #[test]
    fn test_overlap_greater_than_or_equal_to_max_tokens_still_terminates() {
        // step = max_tokens.saturating_sub(overlap), floored to 1 by
        // `core::cmp::max(step, 1)` — this must never loop forever or panic,
        // even when overlap >= max_tokens.
        let chunker = Chunker::new(ChunkConfig::new(2, 5));
        let chunks = chunker.chunk("one two three four five").unwrap();
        assert_eq!(chunks.len(), 5);
    }

    #[test]
    fn test_zero_max_tokens_terminates_without_panicking() {
        let chunker = Chunker::new(ChunkConfig::new(0, 0));
        let chunks = chunker.chunk("one two three").unwrap();
        // Every chunk is empty (end == start), but the walk still finishes.
        assert_eq!(chunks.len(), 3);
        assert!(chunks.iter().all(|c| c.text.is_empty()));
    }
}
