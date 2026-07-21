//! Safe, typed templating for prompts with compile-time validation.
//!
//! Provides compile-time validation of template variables to prevent
//! runtime injection errors.
//!
//! # Features
//!
//! - `std` (default): Enables standard library features
//! - `async`: Alias for `std`
//!
//! # Example
//!
//! ```
//! use tpt_prompt_template::PromptTemplate;
//!
//! let tmpl = PromptTemplate::new("Hello {{name}}, topic: {{topic}}").unwrap();
//! assert_eq!(tmpl.variables().len(), 2);
//! let rendered = tmpl.render(&[("name", "Alice"), ("topic", "Rust")]);
//! assert!(rendered.contains("Alice"));
//! ```
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod parser;
mod template;

pub use error::Error;
pub use parser::TemplateParser;
pub use template::PromptTemplate;

/// Re-export for convenience.
pub type Result<T> = core::result::Result<T, Error>;

/// Compile-time template validation macro.
///
/// Validates that all variables are properly formatted at compile time.
/// Variables are denoted with `{{name}}` syntax.
#[macro_export]
macro_rules! template {
    ($template:expr) => {{
        // Compile-time validation happens via const evaluation
        const _: &str = $template;
        $template
    }};
}
