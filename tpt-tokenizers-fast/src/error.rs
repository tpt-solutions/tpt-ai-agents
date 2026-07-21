use core::fmt;

/// Error type for tpt-tokenizers-fast.
#[derive(Debug)]
pub enum Error {
    /// Vocabulary file error.
    Vocabulary(alloc::string::String),
    /// Encoding error.
    Encoding(alloc::string::String),
    /// Decoding error.
    Decoding(alloc::string::String),
    /// Unknown token.
    UnknownToken(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Vocabulary(msg) => write!(f, "vocabulary error: {msg}"),
            Error::Encoding(msg) => write!(f, "encoding error: {msg}"),
            Error::Decoding(msg) => write!(f, "decoding error: {msg}"),
            Error::UnknownToken(token) => write!(f, "unknown token: {token}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
