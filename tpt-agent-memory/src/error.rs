use core::fmt;

/// Error type for tpt-agent-memory.
#[derive(Debug)]
pub enum Error {
    /// Storage error.
    Storage(alloc::string::String),
    /// Serialization error.
    Serialization(alloc::string::String),
    /// Memory not found.
    NotFound(alloc::string::String),
    /// Concurrent access error.
    Concurrency(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Storage(msg) => write!(f, "storage error: {msg}"),
            Error::Serialization(msg) => write!(f, "serialization error: {msg}"),
            Error::NotFound(msg) => write!(f, "not found: {msg}"),
            Error::Concurrency(msg) => write!(f, "concurrency error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
