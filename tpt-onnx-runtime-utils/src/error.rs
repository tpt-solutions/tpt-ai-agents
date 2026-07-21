use core::fmt;

/// Error type for tpt-onnx-runtime-utils.
#[derive(Debug)]
pub enum Error {
    /// Model loading error.
    ModelLoad(alloc::string::String),
    /// Inference error.
    Inference(alloc::string::String),
    /// Invalid tensor shape.
    InvalidShape { expected: alloc::vec::Vec<usize>, actual: alloc::vec::Vec<usize> },
    /// Type mismatch.
    TypeMismatch(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ModelLoad(msg) => write!(f, "model load error: {msg}"),
            Error::Inference(msg) => write!(f, "inference error: {msg}"),
            Error::InvalidShape { expected, actual } => {
                write!(f, "invalid shape: expected {expected:?}, got {actual:?}")
            }
            Error::TypeMismatch(msg) => write!(f, "type mismatch: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
