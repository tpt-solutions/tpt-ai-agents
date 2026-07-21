# tpt-ai-agents — Release Checklist

Workspace of 11 independent, composable Rust crates for LLM/agent infrastructure. License: `MIT OR Apache-2.0`. Goal: every crate publishable to crates.io at "full rigor" quality.

## 1. Workspace foundation (do first — blocks everything else)

- [ ] Root `Cargo.toml` as virtual workspace: `[workspace] members = [...]`, `resolver = "2"`
- [ ] `[workspace.package]`: shared `edition`, `license = "MIT OR Apache-2.0"`, `repository`, `authors`
- [ ] `[workspace.dependencies]` for shared deps (serde, tokio, etc.) so members inherit versions
- [ ] `LICENSE-MIT` and `LICENSE-APACHE` at root
- [ ] `rust-toolchain.toml` pinning MSRV
- [ ] `rustfmt.toml`
- [ ] `clippy.toml` (deny warnings in CI)
- [ ] `deny.toml` for `cargo-deny` (license allowlist, advisory DB, banned duplicate versions)
- [ ] `.gitignore` (`/target`, etc.)
- [ ] Root `README.md` — workspace overview + crate table + links
- [ ] `CONTRIBUTING.md`
- [ ] GitHub Actions CI:
  - [ ] `cargo fmt --check`
  - [ ] `cargo clippy --all-features --all-targets -D warnings`
  - [ ] Test matrix: stable + MSRV
  - [ ] Per-crate `--no-default-features` build (true `no_std` check)
  - [ ] `cargo doc --all-features` (docs.rs simulation)
  - [ ] `cargo deny check`
  - [ ] `cargo semver-checks` (after first publish of each crate)
  - [ ] `cargo publish --dry-run` gate on release tags
- [ ] Document publish order (dependency tiers):
  - **Tier 0** (no internal deps): `tpt-tokenizers-fast`, `tpt-llm-client-core`, `tpt-vector-store-traits`, `tpt-prompt-template`, `tpt-tool-use-macros`, `tpt-onnx-runtime-utils`, `tpt-ai-mock-server`
  - **Tier 1** (depend on Tier 0): `tpt-rag-pipeline` (→ tokenizers-fast, vector-store-traits, llm-client-core), `tpt-agent-memory` (→ vector-store-traits), `tpt-eval-harness` (→ llm-client-core; ai-mock-server as dev-dependency)
- [ ] Confirm all 11 crate names are unclaimed on crates.io before first publish

## 2. Per-crate checklist

Apply this template to each crate below. Common items (repeat per crate):

- [ ] `cargo new --lib` scaffold, added to workspace `members`
- [ ] `Cargo.toml` metadata: `description`, `license.workspace = true`, `repository`, `documentation`, `readme = "README.md"`, `keywords` (≤5), `categories` (valid crates.io slugs)
- [ ] `#![no_std]` default in `lib.rs` + `extern crate alloc;`, with `std` / `async`|`tokio` / `serde` feature flags; clean `#[cfg(feature = "std")]` / `#[cfg(not(feature = "std"))]` split; no bare `std::` types in core logic (custom error enums instead)
- [ ] Core implementation matching the crate's AI Instruction (below)
- [ ] Unit tests + doctests; `std`-gated integration tests where I/O is needed
- [ ] Doc comments (`///`) on every exported item; crate-level `//!` doc with usage example; README synced into `lib.rs` (`#![doc = include_str!("../README.md")]` or `cargo-rdme`)
- [ ] `CHANGELOG.md` (Keep a Changelog format, starting at `Unreleased`)
- [ ] `cargo clippy --all-features -D warnings` and `cargo fmt --check` pass
- [ ] Builds with `--no-default-features` and with `--all-features`
- [ ] `cargo deny check` clean
- [ ] `cargo publish --dry-run` passes

### tpt-llm-client-core
Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama.
- [ ] Robust SSE parser handling partial/chunked JSON
- [ ] Automatic network retries
- [ ] `std`/`async` feature gates real networking (reqwest/hyper); core parsing stays `no_std`-feasible
- [ ] All common-checklist items above

### tpt-vector-store-traits
Standardized async traits for Qdrant, Milvus, pgvector.
- [ ] Traits defined with GATs for distance metrics and payload types, avoiding boxing
- [ ] All common-checklist items above

### tpt-rag-pipeline
Chunking, embedding batching, context-window management.
- [ ] Streaming chunker respecting token limits (via `tpt-tokenizers-fast`)
- [ ] Overlapping window support
- [ ] Embedding batching + context-window management
- [ ] All common-checklist items above

### tpt-tool-use-macros
Proc macros exposing Rust functions as LLM tools.
- [ ] JSON schema generation from Rust function signatures
- [ ] Automatic serialization/deserialization of tool arguments
- [ ] `proc-macro2`/`syn`/`quote` based; `trybuild` tests for macro output/errors
- [ ] All common-checklist items above

### tpt-prompt-template
Safe, typed templating for prompts.
- [ ] Compile-time validation of template variables (prevents runtime injection errors)
- [ ] All common-checklist items above

### tpt-onnx-runtime-utils
High-level wrappers for local ONNX inference.
- [ ] Zero-copy tensor passing between Rust host and ONNX runtime
- [ ] All common-checklist items above

### tpt-tokenizers-fast
Lightweight, `no_std`-compatible BPE/SentencePiece tokenization.
- [ ] Optimized for edge deployment; minimal allocations in hot path
- [ ] All common-checklist items above

### tpt-agent-memory
In-memory and persistent graph/vector memory for multi-turn state.
- [ ] Concurrent, thread-safe memory store
- [ ] Semantic search support
- [ ] Temporal decay support
- [ ] All common-checklist items above

### tpt-eval-harness
Tools for running LLM eval datasets deterministically.
- [ ] Parallel execution engine
- [ ] Tracks token usage, latency
- [ ] Scores outputs against ground truth
- [ ] All common-checklist items above

### tpt-ai-mock-server
Mock server for testing LLM integrations.
- [ ] Lightweight HTTP server serving pre-recorded SSE streams
- [ ] Validates incoming request schemas
- [ ] All common-checklist items above

## 3. Cross-cutting / release process

- [ ] Verify no circular workspace dependencies (`cargo tree` per crate)
- [ ] Initial version `0.1.0` for all crates; semver policy documented in root README
- [ ] Tag-triggered release workflow (e.g. `cargo-release`, or manual: bump versions per tier, `cargo publish --dry-run` then real publish per tier, wait for crates.io propagation between Tier 0 → Tier 1)
- [ ] Post-publish: confirm docs.rs builds succeed for every crate (all feature combos)
- [ ] Badges (crates.io version, docs.rs, CI, license) on every crate README + root README
- [ ] Stretch goal (not a publish blocker): root `/examples` demonstrating cross-crate usage (e.g. llm-client-core + rag-pipeline + agent-memory)
