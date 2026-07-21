//! In-memory and persistent graph/vector memory for multi-turn state.
//!
//! Provides a memory store with semantic search and temporal decay support.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//!
//! # Example
//!
//! ```
//! use tpt_agent_memory::{MemoryStore, MemoryEntry};
//!
//! let mut store = MemoryStore::new();
//! store.insert(MemoryEntry::new("user_prefers_dark_mode", &["pref"]));
//! let results = store.search("dark mode");
//! assert_eq!(results.len(), 1);
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod decay;
mod entry;
mod error;
mod graph;
mod search;
mod store;

pub use decay::TemporalDecay;
pub use entry::MemoryEntry;
pub use error::Error;
pub use graph::MemoryGraph;
pub use search::SearchQuery;
pub use store::MemoryStore;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
