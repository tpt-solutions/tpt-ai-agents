//! Qdrant adapter for [`tpt_vector_store_traits::VectorStore`].
//!
//! [`QdrantVectorStore`] implements the `VectorStore` trait against a real
//! Qdrant instance via `qdrant-client`, so it can be dropped straight into
//! `tpt_agent_memory::VectorBackedMemoryStore` (behind that crate's
//! `vector-store` feature) for embedding search backed by a real vector
//! database instead of an in-memory scan.
//!
//! # Features
//!
//! - `std` (default): Enables the client itself (networking via
//!   `qdrant-client`/`tonic`). Without it, only the crate's `Error` type is
//!   available.
//! - `async`: Alias for `std`
//!
//! # Example
//!
//! ```no_run
//! use tpt_vector_store_qdrant::QdrantVectorStore;
//! use tpt_vector_store_traits::{Query, VectorStore};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let store = QdrantVectorStore::new("http://localhost:6334", "my_collection").await?;
//! let results = store
//!     .search(&Query {
//!         vector: vec![0.1, 0.2, 0.3],
//!         filter: None,
//!         limit: 5,
//!         offset: None,
//!         include_payload: Some(true),
//!     })
//!     .await?;
//! println!("{} results", results.len());
//! # Ok(())
//! # }
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;

pub use error::Error;

#[cfg(feature = "std")]
mod client;

#[cfg(feature = "std")]
pub use client::QdrantVectorStore;
