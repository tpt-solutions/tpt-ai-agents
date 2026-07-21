use core::fmt;

/// Error type for tpt-ai-mock-server.
#[derive(Debug)]
pub enum Error {
    /// IO error.
    #[cfg(feature = "std")]
    Io(std::io::Error),
    /// JSON error.
    Json(serde_json::Error),
    /// Request validation error.
    Validation(alloc::string::String),
    /// Server startup error.
    Startup(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "std")]
            Error::Io(e) => write!(f, "io error: {e}"),
            Error::Json(e) => write!(f, "json error: {e}"),
            Error::Validation(msg) => write!(f, "validation error: {msg}"),
            Error::Startup(msg) => write!(f, "startup error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}
