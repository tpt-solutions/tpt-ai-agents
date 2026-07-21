use core::fmt;

/// Error type for tpt-prompt-template.
#[derive(Debug)]
pub enum Error {
    /// Unclosed variable delimiter.
    UnclosedVariable { position: usize },
    /// Invalid variable name.
    InvalidVariableName(alloc::string::String),
    /// Missing required variable.
    MissingVariable(alloc::string::String),
    /// Template parsing error.
    ParseError(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnclosedVariable { position } => {
                write!(f, "unclosed variable at position {position}")
            }
            Error::InvalidVariableName(name) => write!(f, "invalid variable name: {name}"),
            Error::MissingVariable(name) => write!(f, "missing variable: {name}"),
            Error::ParseError(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
