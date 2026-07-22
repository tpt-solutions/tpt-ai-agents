use core::fmt;

/// Error type for tpt-llm-client-core.
#[derive(Debug)]
pub enum Error {
    /// Network/transport error.
    #[cfg(feature = "std")]
    Network(reqwest::Error),
    /// JSON parsing error.
    Json(serde_json::Error),
    /// Invalid SSE format.
    SseFormat(alloc::string::String),
    /// Request validation error.
    InvalidRequest(alloc::string::String),
    /// Provider-specific error.
    Provider {
        code: u16,
        message: alloc::string::String,
    },
    /// Rate limit exceeded.
    RateLimited,
    /// Authentication error.
    Unauthorized,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "std")]
            Error::Network(e) => write!(f, "network error: {e}"),
            Error::Json(e) => write!(f, "json error: {e}"),
            Error::SseFormat(msg) => write!(f, "sse format error: {msg}"),
            Error::InvalidRequest(msg) => write!(f, "invalid request: {msg}"),
            Error::Provider { code, message } => write!(f, "provider error {code}: {message}"),
            Error::RateLimited => write!(f, "rate limited"),
            Error::Unauthorized => write!(f, "unauthorized"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}
