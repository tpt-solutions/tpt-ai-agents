use alloc::string::String;
use core::fmt;

/// Error type for [`crate::PgVectorStore`].
#[derive(Debug)]
pub enum Error {
    /// The underlying `sqlx` call failed.
    Sql(String),
    /// A `tpt_vector_store_traits::Error` bubbled up through the trait.
    Store(tpt_vector_store_traits::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Sql(msg) => write!(f, "pgvector: {msg}"),
            Error::Store(e) => write!(f, "pgvector: {e}"),
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
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Sql(alloc::string::ToString::to_string(&e))
    }
}
