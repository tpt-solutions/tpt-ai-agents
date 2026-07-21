# tpt-ai-agents — Release Checklist

Workspace of 10 independent, composable Rust crates for LLM/agent infrastructure. License: `MIT OR Apache-2.0`. Goal: every crate publishable to crates.io at "full rigor" quality.

## 1. Workspace foundation

- [x] Root `Cargo.toml` as virtual workspace
- [x] `[workspace.package]` and `[workspace.dependencies]`
- [x] `LICENSE-MIT` and `LICENSE-APACHE` at root
- [x] `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`, `deny.toml`
- [x] `.gitignore`
- [x] Root `README.md` with crate table + badges
- [x] `CONTRIBUTING.md` (includes "adding a new crate" section)
- [x] GitHub Actions CI (`ci.yml`) — fmt, clippy, test matrix, no_std, doc, deny, semver, publish-dry-run
- [x] Tag-triggered release workflow (`release.yml`) — version verification, tiered publish, GitHub Release
- [x] Document publish order (dependency tiers)
- [ ] Confirm all 10 crate names are unclaimed on crates.io before first publish

## 2. Per-crate checklist

- [x] `cargo new --lib` scaffold, added to workspace `members`
- [x] `Cargo.toml` metadata complete (including version on path deps for publish)
- [x] `#![no_std]` default with `extern crate alloc;` and feature flags
- [x] Core implementation (structural — real backends are stubs, see Section 4)
- [x] Unit tests + doctests (38 tests across all crates)
- [x] Doc comments on every exported item; crate-level `//!` doc with working examples
- [x] `CHANGELOG.md` (Keep a Changelog format) — all 10 crates + root
- [x] `cargo clippy --all-features -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] Builds with `--no-default-features` and with `--all-features`
- [x] `cargo deny check` passes (advisories ok, bans ok, licenses ok, sources ok)
- [x] `cargo publish --dry-run` passes for all Tier 0 crates
- [x] Tier 1 crates correctly fail dry-run (dependencies not yet on crates.io — expected)

### tpt-llm-client-core
- [x] Robust SSE parser (`SseParser::feed`/`flush`, 3 tests)
- [ ] Automatic network retries (not yet implemented)
- [x] `std`/`async` feature gates real networking

### tpt-vector-store-traits
- [x] Traits with GATs for distance metrics and payload types
- [x] 3 tests

### tpt-rag-pipeline
- [x] Streaming chunker respecting token limits
- [x] Overlapping window support (2 tests)
- [x] Embedding batching + context-window management (basic data holders)

### tpt-tool-use-macros
- [x] JSON schema generation from Rust function signatures
- [ ] Automatic serialization/deserialization of tool arguments
- [x] 2 integration tests

### tpt-prompt-template
- [x] Compile-time validation of template variables (2 tests + 1 doctest)

### tpt-onnx-runtime-utils
- [x] Tensor and session abstractions (mock backend)
- [x] 6 tests

### tpt-tokenizers-fast
- [x] BPE tokenizer with no_std support
- [x] 5 tests

### tpt-agent-memory
- [x] Memory store with semantic search (1 test + 1 doctest)
- [x] Graph with validated edges and neighbor lookup (4 tests)
- [x] Temporal decay (1 test)
- [x] Unique ID generation (2 tests)

### tpt-eval-harness
- [x] Evaluation harness with scorer trait (1 test + 1 doctest)
- [x] Parallel executor with batch processing (1 test)

### tpt-ai-mock-server
- [x] Request validation (2 tests)

## 3. Cross-cutting / release process

- [x] Initial version `0.1.0` for all crates
- [x] Badges on every crate README + root README
- [x] Root `/examples/` integration example (rag-pipeline + agent-memory)
- [x] Root `CHANGELOG.md`
- [x] Verify no circular workspace dependencies (`cargo tree` per crate)
- [x] Tag-triggered release workflow (`.github/workflows/release.yml`)
- [x] `cargo deny check` passes locally
- [x] `cargo publish --dry-run` passes for all Tier 0 crates
- [x] Doc fixes: agent-memory store doc, eval-harness doc example
- [ ] Confirm all 10 crate names are unclaimed on crates.io
- [ ] Post-publish: confirm docs.rs builds succeed

## 4. Remaining feature gaps (not publish blockers)

### 4.1 Known stubs
- `tpt-llm-client-core`: No real HTTP/networking code yet (SSE parser works)
- `tpt-tokenizers-fast`: BPE encode does character lookup, not real merge application
- `tpt-onnx-runtime-utils`: `Session::run()` is a passthrough (no real ONNX inference)
- `tpt-agent-memory`: `MemoryStore` is not thread-safe (plain `BTreeMap`)
- `tpt-eval-harness`: No actual file-based dataset loading (works with in-memory samples)
- `tpt-tool-use-macros`: No automatic serde for tool arguments yet

### 4.2 Future work
- Root integration example with `tpt-ai-mock-server` for CI-testable demos
- `cargo-generate` template for new crate scaffolding
- CI job to detect `rust,ignore`/`unimplemented!()`/`todo!()` regressions
- Per-crate maturity indicators in root README
- `GETTING_STARTED.md` and `docs/ARCHITECTURE.md`
