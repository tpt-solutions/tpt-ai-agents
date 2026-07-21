//! Tools for running LLM eval datasets deterministically.
//!
//! Provides a parallel execution engine that tracks token usage, latency,
//! and scores outputs against ground truth.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//!
//! # Example
//!
//! ```rust,ignore
//! use tpt_eval_harness::{EvalHarness, EvalConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let harness = EvalHarness::new(EvalConfig::default());
//!     let results = harness.run("eval_dataset.jsonl").await.unwrap();
//!     println!("Accuracy: {}", results.accuracy());
//! }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod config;
mod error;
mod evaluator;
mod executor;
mod metrics;
mod scoring;

pub use config::EvalConfig;
pub use error::Error;
pub use evaluator::{EvalHarness, EvalSample};
pub use executor::ParallelExecutor;
pub use metrics::{EvalMetrics, TokenUsage};
pub use scoring::{Score, Scorer};

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
