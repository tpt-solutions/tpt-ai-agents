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

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0.
