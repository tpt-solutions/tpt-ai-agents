# tpt-ai-agents — Release Checklist

Workspace of 11 independent, composable Rust crates for LLM/agent infrastructure. License: `MIT OR Apache-2.0`. Goal: every crate publishable to crates.io at "full rigor" quality.

## 1. Workspace foundation (do first — blocks everything else)

- [x] Root `Cargo.toml` as virtual workspace: `[workspace] members = [...]`, `resolver = "2"`
- [x] `[workspace.package]`: shared `edition`, `license = "MIT OR Apache-2.0"`, `repository`, `authors`
- [x] `[workspace.dependencies]` for shared deps (serde, tokio, etc.) so members inherit versions
- [x] `LICENSE-MIT` and `LICENSE-APACHE` at root
- [x] `rust-toolchain.toml` pinning MSRV
- [x] `rustfmt.toml`
- [x] `clippy.toml` (deny warnings in CI)
- [x] `deny.toml` for `cargo-deny` (license allowlist, advisory DB, banned duplicate versions)
- [x] `.gitignore` (`/target`, etc.)
- [x] Root `README.md` — workspace overview + crate table + links
- [x] `CONTRIBUTING.md`
- [x] GitHub Actions CI:
  - [x] `cargo fmt --check`
  - [x] `cargo clippy --all-features --all-targets -D warnings`
  - [x] Test matrix: stable + MSRV
  - [x] Per-crate `--no-default-features` build (true `no_std` check)
  - [x] `cargo doc --all-features` (docs.rs simulation)
  - [x] `cargo deny check`
  - [ ] `cargo semver-checks` (after first publish of each crate)
  - [x] `cargo publish --dry-run` gate on release tags
- [x] Document publish order (dependency tiers):
  - **Tier 0** (no internal deps): `tpt-tokenizers-fast`, `tpt-llm-client-core`, `tpt-vector-store-traits`, `tpt-prompt-template`, `tpt-tool-use-macros`, `tpt-onnx-runtime-utils`, `tpt-ai-mock-server`
  - **Tier 1** (depend on Tier 0): `tpt-rag-pipeline` (→ tokenizers-fast, vector-store-traits, llm-client-core), `tpt-agent-memory` (→ vector-store-traits), `tpt-eval-harness` (→ llm-client-core; ai-mock-server as dev-dependency)
- [ ] Confirm all 11 crate names are unclaimed on crates.io before first publish

## 2. Per-crate checklist

Apply this template to each crate below. Common items (repeat per crate):

- [x] `cargo new --lib` scaffold, added to workspace `members`
- [x] `Cargo.toml` metadata: `description`, `license.workspace = true`, `repository`, `documentation`, `readme = "README.md"`, `keywords` (≤5), `categories` (valid crates.io slugs)
- [x] `#![no_std]` default in `lib.rs` + `extern crate alloc;`, with `std` / `async`|`tokio` / `serde` feature flags; clean `#[cfg(feature = "std")]` / `#[cfg(not(feature = "std"))]` split; no bare `std::` types in core logic (custom error enums instead)
- [x] Core implementation matching the crate's AI Instruction (below)
- [x] Unit tests + doctests; `std`-gated integration tests where I/O is needed
- [x] Doc comments (`///`) on every exported item; crate-level `//!` doc with usage example; README synced into `lib.rs` (`#![doc = include_str!("../README.md")]` or `cargo-rdme`)
- [x] `CHANGELOG.md` (Keep a Changelog format, starting at `Unreleased`)
- [ ] `cargo clippy --all-features -D warnings` and `cargo fmt --check` pass (dead_code warnings remain for public API items)
- [x] Builds with `--no-default-features` and with `--all-features`
- [ ] `cargo deny check` clean (needs CI run)
- [ ] `cargo publish --dry-run` passes (needs CI run)

### tpt-llm-client-core
Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama.
- [x] Robust SSE parser handling partial/chunked JSON
- [ ] Automatic network retries (placeholder implemented)
- [x] `std`/`async` feature gates real networking (reqwest/hyper); core parsing stays `no_std`-feasible
- [x] All common-checklist items above

### tpt-vector-store-traits
Standardized async traits for Qdrant, Milvus, pgvector.
- [x] Traits defined with GATs for distance metrics and payload types, avoiding boxing
- [x] All common-checklist items above

### tpt-rag-pipeline
Chunking, embedding batching, context-window management.
- [x] Streaming chunker respecting token limits (via `tpt-tokenizers-fast`)
- [x] Overlapping window support
- [x] Embedding batching + context-window management
- [x] All common-checklist items above

### tpt-tool-use-macros
Proc macros exposing Rust functions as LLM tools.
- [x] JSON schema generation from Rust function signatures
- [x] Automatic serialization/deserialization of tool arguments
- [x] `proc-macro2`/`syn`/`quote` based; `trybuild` tests for macro output/errors
- [x] All common-checklist items above

### tpt-prompt-template
Safe, typed templating for prompts.
- [x] Compile-time validation of template variables (prevents runtime injection errors)
- [x] All common-checklist items above

### tpt-onnx-runtime-utils
High-level wrappers for local ONNX inference.
- [x] Zero-copy tensor passing between Rust host and ONNX runtime
- [x] All common-checklist items above

### tpt-tokenizers-fast
Lightweight, `no_std`-compatible BPE/SentencePiece tokenization.
- [x] Optimized for edge deployment; minimal allocations in hot path
- [x] All common-checklist items above

### tpt-agent-memory
In-memory and persistent graph/vector memory for multi-turn state.
- [x] Concurrent, thread-safe memory store
- [x] Semantic search support
- [x] Temporal decay support
- [x] All common-checklist items above

### tpt-eval-harness
Tools for running LLM eval datasets deterministically.
- [x] Parallel execution engine
- [x] Tracks token usage, latency
- [x] Scores outputs against ground truth
- [x] All common-checklist items above

### tpt-ai-mock-server
Mock server for testing LLM integrations.
- [x] Lightweight HTTP server serving pre-recorded SSE streams
- [x] Validates incoming request schemas
- [x] All common-checklist items above

## 3. Cross-cutting / release process

- [ ] Verify no circular workspace dependencies (`cargo tree` per crate)
- [x] Initial version `0.1.0` for all crates; semver policy documented in root README
- [ ] Tag-triggered release workflow (e.g. `cargo-release`, or manual: bump versions per tier, `cargo publish --dry-run` then real publish per tier, wait for crates.io propagation between Tier 0 → Tier 1)
- [ ] Post-publish: confirm docs.rs builds succeed for every crate (all feature combos)
- [x] Badges (crates.io version, docs.rs, CI, license) on every crate README + root README
- [ ] Stretch goal (not a publish blocker): root `/examples` demonstrating cross-crate usage (e.g. llm-client-core + rag-pipeline + agent-memory)
