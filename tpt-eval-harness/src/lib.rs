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
//! ```
//! use tpt_eval_harness::{EvalHarness, EvalConfig, EvalSample};
//!
//! let harness = EvalHarness::new(EvalConfig::default());
//! let samples = vec![
//!     EvalSample::new("What is 2+2?", "4", "4"),
//!     EvalSample::new("Capital of France?", "Paris", "Lyon"),
//! ];
//! let metrics = harness.run(&samples);
//! assert_eq!(metrics.correct, 1);
//! ```
//!
//! Datasets can also be loaded from a JSONL file (one JSON-encoded
//! [`EvalSample`] per line), with the `std` feature enabled:
//!
//! ```no_run
//! use tpt_eval_harness::{EvalHarness, EvalConfig};
//!
//! let harness = EvalHarness::new(EvalConfig::default());
//! let metrics = harness.run_file("eval_dataset.jsonl").unwrap();
//! println!("accuracy: {}", metrics.accuracy());
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
