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
//! ```rust,ignore
//! use tpt_tokenizers_fast::{BpeTokenizer, Tokenizer};
//!
//! fn tokenize() {
//!     let tokenizer = BpeTokenizer::from_file("vocab.bpe").unwrap();
//!     let tokens = tokenizer.encode("Hello, world!").unwrap();
//!     println!("{:?}", tokens);
//! }
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
    fn decode(&self, tokens: &[TokenId]) -> core::result::Result<alloc::string::String, Self::Error>;
    fn vocab_size(&self) -> usize;
}

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;
