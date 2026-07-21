use core::fmt;

/// Error type for tpt-eval-harness.
#[derive(Debug)]
pub enum Error {
    /// Dataset loading error.
    Dataset(alloc::string::String),
    /// Evaluation error.
    Evaluation(alloc::string::String),
    /// Scoring error.
    Scoring(alloc::string::String),
    /// Configuration error.
    Config(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dataset(msg) => write!(f, "dataset error: {msg}"),
            Error::Evaluation(msg) => write!(f, "evaluation error: {msg}"),
            Error::Scoring(msg) => write!(f, "scoring error: {msg}"),
            Error::Config(msg) => write!(f, "config error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
