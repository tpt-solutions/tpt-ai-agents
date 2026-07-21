use core::fmt;

/// Error type for tpt-vector-store-traits.
#[derive(Debug)]
pub enum Error {
    /// Connection error.
    Connection(alloc::string::String),
    /// Collection not found.
    NotFound(alloc::string::String),
    /// Invalid vector dimension.
    DimensionMismatch { expected: usize, actual: usize },
    /// Serialization error.
    Serialization(alloc::string::String),
    /// Timeout.
    Timeout,
    /// Provider-specific error.
    Provider { code: u16, message: alloc::string::String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Connection(msg) => write!(f, "connection error: {msg}"),
            Error::NotFound(msg) => write!(f, "not found: {msg}"),
            Error::DimensionMismatch { expected, actual } => {
                write!(f, "dimension mismatch: expected {expected}, got {actual}")
            }
            Error::Serialization(msg) => write!(f, "serialization error: {msg}"),
            Error::Timeout => write!(f, "timeout"),
            Error::Provider { code, message } => write!(f, "provider error {code}: {message}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
