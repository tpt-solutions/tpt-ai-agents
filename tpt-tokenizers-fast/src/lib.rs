//! Lightweight, no_std-compatible BPE/SentencePiece tokenization.
//!
//! Optimized for edge deployment with minimal allocations in the hot path.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//!
//! # Example
//!
//! ```
//! use tpt_tokenizers_fast::{BpeTokenizer, Vocabulary};
//! use std::collections::BTreeMap;
//!
//! let mut vocab = Vocabulary::new();
//! for (i, ch) in "helo, wrd!".chars().enumerate() {
//!     vocab.insert(&ch.to_string(), i as u32);
//! }
//! let tokenizer = BpeTokenizer::new(vocab, BTreeMap::new());
//! let tokens = tokenizer.encode("hello").unwrap();
//! assert_eq!(tokens.len(), 5);
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod bpe;
mod error;
mod token;
mod vocab;

pub use bpe::BpeTokenizer;
pub use error::Error;
pub use token::{Token, TokenId};
pub use vocab::Vocabulary;

/// Core tokenizer trait.
pub trait Tokenizer {
    type Error;

    fn encode(&self, text: &str) -> core::result::Result<alloc::vec::Vec<Token>, Self::Error>;
    fn decode(
        &self,
        tokens: &[TokenId],
    ) -> core::result::Result<alloc::string::String, Self::Error>;
    fn vocab_size(&self) -> usize;
}

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
