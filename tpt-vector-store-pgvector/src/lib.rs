//! PostgreSQL/pgvector adapter for [`tpt_vector_store_traits::VectorStore`].
//!
//! [`PgVectorStore`] implements the `VectorStore` trait against a PostgreSQL
//! database with the [pgvector](https://github.com/pgvector/pgvector)
//! extension installed. It uses `sqlx` for async database access and the
//! `pg-vector` crate for the `vector` column type.
//!
//! # Features
//!
//! - `std` (default): Enables the adapter itself (requires `sqlx` and
//!   `pg-vector`). Without it, only the crate's `Error` type is available.
//! - `async`: Alias for `std`
//!
//! # Requirements
//!
//! - A running PostgreSQL instance with the `vector` extension enabled
//! - A table with the expected schema (see [`PgVectorStore`] docs)
//!
//! # Example
//!
//! ```no_run
//! use tpt_vector_store_pgvector::PgVectorStore;
//! use tpt_vector_store_traits::{Query, VectorStore};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let store = PgVectorStore::new(
//!     "postgres://user:pass@localhost/mydb",
//!     "documents",
//!     384,
//! ).await?;
//! let results = store
//!     .search(&Query {
//!         vector: vec![0.1; 384],
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
pub use client::PgVectorStore;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_error_display() {
        let err = Error::Sql("connection refused".into());
        assert!(err.to_string().contains("pgvector"));
    }

    #[test]
    fn test_store_error_conversion() {
        let err: Error = tpt_vector_store_traits::Error::Timeout.into();
        assert!(err.to_string().contains("timeout"));
    }
}
