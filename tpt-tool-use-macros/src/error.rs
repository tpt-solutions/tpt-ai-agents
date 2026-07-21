use core::fmt;

/// Error type for tpt-tool-use-macros.
#[derive(Debug)]
pub enum Error {
    /// Invalid function signature.
    InvalidSignature(alloc::string::String),
    /// Unsupported type.
    UnsupportedType(alloc::string::String),
    /// Missing required attribute.
    MissingAttribute(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSignature(msg) => write!(f, "invalid signature: {msg}"),
            Error::UnsupportedType(msg) => write!(f, "unsupported type: {msg}"),
            Error::MissingAttribute(msg) => write!(f, "missing attribute: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
