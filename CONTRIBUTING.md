# Contributing to tpt-ai-agents

Thanks for your interest in contributing!

## Getting Started

1. Fork and clone the repository
2. Install Rust stable via `rustup` (see `rust-toolchain.toml` for version)
3. Run `cargo test --all` to verify everything works
4. Create a feature branch

## Code Style

- `cargo fmt` before committing
- `cargo clippy --all-features --all-targets -D warnings` must pass
- `#![no_std]` by default in all crates; use feature flags for `std`
- Doc comments (`///`) on all public items

## Testing

- Unit tests and doctests in each crate
- Integration tests gated behind `#[cfg(feature = "std")]`
- Run the full suite: `cargo test --all --all-features`

## Pull Requests

- Keep PRs focused on one change
- Include a clear description of what and why
- Ensure CI passes
- Update `CHANGELOG.md` for user-facing changes

## Adding a New Crate

1. Create the crate directory: `cargo new --lib tpt-<name>`
2. Add to `Cargo.toml` workspace `members` list
3. Required `Cargo.toml` fields:
   - `name`, `description`, `version = "0.1.0"`
   - `edition.workspace = true`, `license.workspace = true`, `repository.workspace = true`
   - `readme = "README.md"`, `keywords` (≤5), `categories` (valid crates.io slugs)
4. In `lib.rs`: start with `#![no_std]`, `extern crate alloc;`, feature flags for `std`/`async`
5. Create `README.md`, `CHANGELOG.md` (Keep a Changelog format)
6. Add unit tests; doctests should compile and pass
7. Run `cargo clippy --all-features --all-targets -D warnings` and `cargo fmt --check`
8. Determine publish tier (0 = no internal deps, 1+ = depends on lower tiers)

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0.
