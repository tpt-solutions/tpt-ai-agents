use alloc::string::String;
use core::fmt;

/// Error type for [`crate::QdrantVectorStore`].
#[derive(Debug)]
pub enum Error {
    /// The underlying `qdrant-client` call failed.
    Client(String),
    /// A `tpt_vector_store_traits::Error` bubbled up through the trait.
    Store(tpt_vector_store_traits::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Client(msg) => write!(f, "qdrant: {msg}"),
            Error::Store(e) => write!(f, "qdrant: {e}"),
        }
    }
}

impl From<tpt_vector_store_traits::Error> for Error {
    fn from(e: tpt_vector_store_traits::Error) -> Self {
        Error::Store(e)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<qdrant_client::QdrantError> for Error {
    fn from(e: qdrant_client::QdrantError) -> Self {
        Error::Client(alloc::string::ToString::to_string(&e))
    }
}
