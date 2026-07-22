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

### 4.1 Known stubs — all fixed 2026-07-22
- [x] `tpt-llm-client-core`: Real HTTP now implemented (`SseClient::send`/`stream`) for
      OpenAI, Anthropic, and Ollama, including provider-specific request/response mapping
      and real SSE/NDJSON streaming. Covered by 13 unit tests in `src/http.rs`.
- [x] `tpt-tokenizers-fast`: `BpeTokenizer::encode` now applies learned merges in
      priority order instead of doing a 1:1 char lookup.
- [x] `tpt-onnx-runtime-utils`: Real ONNX inference via the pure-Rust `tract-onnx` engine
      (chosen over `ort`/onnxruntime-native to keep the crate buildable offline with no
      system C toolchain). Verified end-to-end with a hand-built minimal ONNX model in
      tests. Without the `std` feature, `run()` now returns an honest error instead of a
      silent zero-filled passthrough.
- [x] `tpt-agent-memory`: Added `ConcurrentMemoryStore` (`Arc<RwLock<MemoryStore>>`,
      `std`-gated) for genuine thread-safe access; `MemoryStore` itself remains the
      single-threaded no_std-compatible base, with doc comments now accurately describing
      which type to use for concurrent access.
- [x] `tpt-eval-harness`: Added `EvalSample::load_jsonl` and `EvalHarness::run_file`
      (std-gated) for real file-based dataset loading, replacing the doc example that
      referenced a non-existent API.
- [x] `tpt-tool-use-macros`: `#[tool]` now generates a `Serialize + Deserialize` `Args`
      struct and a `<fn>_call(json) -> Result<Output, serde_json::Error>` dispatcher.
      Also fixed two real bugs found while implementing this: schema property names were
      derived from the parameter *type* instead of its *name* (e.g. `"string"` instead of
      `"location"`), and the schema builder emitted a trailing comma making it invalid JSON.

### 4.2 Future work
- Root integration example with `tpt-ai-mock-server` for CI-testable demos
- `cargo-generate` template for new crate scaffolding
- CI job to detect `rust,ignore`/`unimplemented!()`/`todo!()` regressions
- Per-crate maturity indicators in root README
- `GETTING_STARTED.md` and `docs/ARCHITECTURE.md`
- End-to-end example wiring `llm-client-core` + `tool-use-macros` + `agent-memory` +
  `rag-pipeline` together now that real HTTP exists
- Reference vector-store adapter example (e.g. Qdrant) for `tpt-vector-store-traits`
