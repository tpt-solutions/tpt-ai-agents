use alloc::string::String;
use core::fmt;

/// Error type for [`crate::EmbeddingsClient`] implementations.
#[derive(Debug)]
pub enum Error {
    /// The underlying HTTP client call failed.
    Network(String),
    /// The provider returned an error response.
    Provider { code: u16, message: String },
    /// A JSON serialization or deserialization error.
    Serialization(String),
    /// The input batch is empty.
    EmptyBatch,
    /// The response contained no embedding data.
    NoData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(msg) => write!(f, "network error: {msg}"),
            Error::Provider { code, message } => write!(f, "provider error {code}: {message}"),
            Error::Serialization(msg) => write!(f, "serialization error: {msg}"),
            Error::EmptyBatch => write!(f, "input batch is empty"),
            Error::NoData => write!(f, "response contained no embedding data"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
