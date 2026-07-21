use core::fmt;

/// Error type for tpt-rag-pipeline.
#[derive(Debug)]
pub enum Error {
    /// Chunking error.
    Chunking(alloc::string::String),
    /// Tokenizer error.
    Tokenizer(alloc::string::String),
    /// Embedding error.
    Embedding(alloc::string::String),
    /// Context window overflow.
    ContextOverflow { tokens: usize, limit: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Chunking(msg) => write!(f, "chunking error: {msg}"),
            Error::Tokenizer(msg) => write!(f, "tokenizer error: {msg}"),
            Error::Embedding(msg) => write!(f, "embedding error: {msg}"),
            Error::ContextOverflow { tokens, limit } => {
                write!(f, "context overflow: {tokens} tokens exceeds limit of {limit}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
