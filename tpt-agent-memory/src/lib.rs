//! In-memory and (with the `std` feature) file-persistent graph/vector
//! memory for multi-turn state.
//!
//! [`MemoryStore::search`] supports two modes:
//! - **Keyword search** (default): substring match on content/tags, ranked
//!   by `MemoryEntry::score`. Works in `no_std`.
//! - **Semantic search**: when a query is built with
//!   [`SearchQuery::with_embedding`], entries that carry a
//!   [`MemoryEntry::embedding`] are ranked by cosine similarity instead.
//!   This crate does not compute embeddings itself — obtain them from an
//!   embedding model (e.g. via `tpt-llm-client-core`) and attach them via
//!   [`MemoryEntry::with_embedding`].
//!
//! # Features
//!
//! - `std` (default): Enables standard library features, including
//!   [`MemoryStore::save_to_file`] / [`MemoryStore::load_from_file`]
//!   (JSON file persistence) and [`ConcurrentMemoryStore`].
//! - `async`: Alias for `std`
//!
//! # Example
//!
//! ```
//! use tpt_agent_memory::{MemoryStore, MemoryEntry, SearchQuery};
//!
//! let mut store = MemoryStore::new();
//! store.insert(MemoryEntry::new("user prefers dark mode", &["pref"]));
//! let query = SearchQuery::new("dark mode");
//! let results = store.search(&query);
//! assert_eq!(results.len(), 1);
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod concurrent;
mod decay;
mod entry;
mod error;
mod graph;
mod search;
mod store;

#[cfg(feature = "std")]
pub use concurrent::ConcurrentMemoryStore;
pub use decay::TemporalDecay;
pub use entry::MemoryEntry;
pub use error::Error;
pub use graph::MemoryGraph;
pub use search::SearchQuery;
pub use store::MemoryStore;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
