#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for {{ crate_name }}.
#[derive(Debug)]
pub enum Error {
    /// Placeholder error variant.
    Placeholder(alloc::string::String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Placeholder(msg) => write!(f, "{msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Error::Placeholder("hello".into());
        match result {
            Error::Placeholder(msg) => assert_eq!(msg, "hello"),
        }
    }
}
