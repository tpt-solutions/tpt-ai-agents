//! High-level wrappers for local ONNX inference.
//!
//! Provides zero-copy tensor passing between the Rust host and ONNX runtime
//! to minimize latency.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//!
//! # Example
//!
//! ```rust,ignore
//! use tpt_onnx_runtime_utils::{Session, Tensor};
//!
//! fn run_inference() -> Result<(), Box<dyn std::error::Error>> {
//!     let session = Session::from_file("model.onnx")?;
//!     let input = Tensor::from_slice(&[1.0, 2.0, 3.0], &[1, 3])?;
//!     let output = session.run(&input)?;
//!     Ok(())
//! }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod session;
mod tensor;

pub use error::Error;
pub use session::Session;
pub use tensor::Tensor;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
