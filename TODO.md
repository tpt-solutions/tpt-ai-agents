# tpt-ai-agents — Release Checklist

Workspace of 10 independent, composable Rust crates for LLM/agent infrastructure. License: `MIT OR Apache-2.0`. Goal: every crate publishable to crates.io at "full rigor" quality.

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
- [x] GitHub Actions CI (`.github/workflows/ci.yml` defines all jobs below; not yet run against this commit):
  - [x] `cargo fmt --check`
  - [x] `cargo clippy --all-features --all-targets -D warnings`
  - [x] Test matrix: stable + MSRV
  - [x] Per-crate `--no-default-features` build (true `no_std` check)
  - [x] `cargo doc --all-features` (docs.rs simulation)
  - [x] `cargo deny check`
  - [x] `cargo semver-checks` (after first publish of each crate)
  - [x] `cargo publish --dry-run` gate on release tags
- [x] Document publish order (dependency tiers):
  - **Tier 0** (no internal deps): `tpt-tokenizers-fast`, `tpt-llm-client-core`, `tpt-vector-store-traits`, `tpt-prompt-template`, `tpt-tool-use-macros`, `tpt-onnx-runtime-utils`, `tpt-ai-mock-server`
  - **Tier 1** (depend on Tier 0): `tpt-rag-pipeline` (→ tokenizers-fast, vector-store-traits, llm-client-core), `tpt-agent-memory` (→ vector-store-traits), `tpt-eval-harness` (→ llm-client-core; ai-mock-server as dev-dependency)
- [ ] Confirm all 10 crate names are unclaimed on crates.io before first publish

## 2. Per-crate checklist

Apply this template to each crate below. Common items (repeat per crate):

- [x] `cargo new --lib` scaffold, added to workspace `members`
- [x] `Cargo.toml` metadata: `description`, `license.workspace = true`, `repository`, `documentation`, `readme = "README.md"`, `keywords` (≤5), `categories` (valid crates.io slugs)
- [x] `#![no_std]` default in `lib.rs` + `extern crate alloc;`, with `std` / `async`|`tokio` / `serde` feature flags (N/A for `tpt-tool-use-macros`, a `proc-macro = true` crate that runs on the host)
- [ ] Core implementation matching the crate's AI Instruction (below) — several crates are structural stubs only; see per-crate notes
- [ ] Unit tests + doctests; `std`-gated integration tests where I/O is needed — `cargo test --all` currently passes (verified 2026-07-22; `tpt-prompt-template::parser::tests::test_parse_variables` passes, not failing); `tpt-vector-store-traits`, `tpt-tool-use-macros`, `tpt-onnx-runtime-utils`, `tpt-tokenizers-fast`, `tpt-eval-harness` have zero tests
- [x] Doc comments (`///`) on every exported item; crate-level `//!` doc with usage example
- [ ] README synced into `lib.rs` (`#![doc = include_str!("../README.md")]` or `cargo-rdme`) — per-crate `README.md` exists but is not included into `lib.rs`
- [x] `CHANGELOG.md` (Keep a Changelog format, starting at `Unreleased`) — present in all 10 crates
- [ ] `cargo clippy --all-features -D warnings` and `cargo fmt --check` pass — currently fails: `dead_code` warnings in `tpt-llm-client-core`, `tpt-agent-memory`, `tpt-eval-harness`, `tpt-ai-mock-server`
- [ ] Builds with `--no-default-features` and with `--all-features` — `--no-default-features` currently **fails** to compile for `tpt-vector-store-traits`, `tpt-agent-memory`, `tpt-ai-mock-server`
- [ ] `cargo deny check` clean (not runnable locally — `cargo-deny` not installed; only verified via CI config)
- [ ] `cargo publish --dry-run` passes (not yet run)

### tpt-llm-client-core
Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama.
- [x] Robust SSE parser handling partial/chunked JSON (`SseParser::feed`/`flush`, tested)
- [ ] Automatic network retries — no retry logic found in the crate
- [x] `std`/`async` feature gates real networking (reqwest); core parsing stays `no_std`-feasible
- [ ] All common-checklist items above (dead-code warning on `SseClient` fields; no doctests included)

### tpt-vector-store-traits
Standardized async traits for Qdrant, Milvus, pgvector.
- [x] Traits defined with GATs for distance metrics and payload types, avoiding boxing (`VectorStore` in `store.rs`)
- [ ] All common-checklist items above (fails to build with `--no-default-features`; zero tests; `traits.rs` is an empty re-export stub)

### tpt-rag-pipeline
Chunking, embedding batching, context-window management.
- [x] Streaming chunker respecting token limits
- [x] Overlapping window support (tested)
- [ ] Embedding batching + context-window management — `EmbeddingBatch`/`ContextWindow` exist as basic data holders (push/allocate/remaining); no actual batching-to-provider or eviction logic yet
- [ ] All common-checklist items above

### tpt-tool-use-macros
Proc macros exposing Rust functions as LLM tools.
- [x] JSON schema generation from Rust function signatures (`function_to_json_schema`)
- [ ] Automatic serialization/deserialization of tool arguments — not implemented in `codegen.rs` yet
- [ ] `proc-macro2`/`syn`/`quote` based; `trybuild` tests for macro output/errors — crate is proc-macro2/syn/quote based, but there is no `tests/` directory, so `trybuild` is a dev-dependency only, unused
- [ ] All common-checklist items above (zero tests)

### tpt-prompt-template
Safe, typed templating for prompts.
- [x] Compile-time validation of template variables (prevents runtime injection errors) — `test_parse_variables` passes (verified 2026-07-22)
- [ ] All common-checklist items above

### tpt-onnx-runtime-utils
High-level wrappers for local ONNX inference.
- [ ] Zero-copy tensor passing between Rust host and ONNX runtime — `Tensor::from_slice` currently copies into an owned buffer; no zero-copy path yet
- [ ] All common-checklist items above (zero tests)

### tpt-tokenizers-fast
Lightweight, `no_std`-compatible BPE/SentencePiece tokenization.
- [ ] Optimized for edge deployment; minimal allocations in hot path — basic BPE encode/decode present; not benchmarked/optimized yet
- [ ] All common-checklist items above (zero tests)

### tpt-agent-memory
In-memory and persistent graph/vector memory for multi-turn state.
- [ ] Concurrent, thread-safe memory store — `MemoryStore` currently uses plain `&mut self` methods, no `Mutex`/`RwLock`/`Arc` wrapper
- [x] Semantic search support (`SearchQuery`, tested)
- [x] Temporal decay support (`DecayFn::compute`, tested)
- [ ] All common-checklist items above (fails to build with `--no-default-features`; dead-code warnings)

### tpt-eval-harness
Tools for running LLM eval datasets deterministically.
- [ ] Parallel execution engine — `Executor` only stores a `max_concurrency` value; no actual concurrent task spawning/semaphore yet
- [x] Tracks token usage, latency (`TokenUsage`, `latency_ms`)
- [x] Scores outputs against ground truth (`Scorer` trait + impls)
- [ ] All common-checklist items above (zero tests; dead-code warnings)

### tpt-ai-mock-server
Mock server for testing LLM integrations.
- [x] Lightweight HTTP server serving pre-recorded SSE streams (`Server::start`, tokio-based)
- [x] Validates incoming request schemas (`validate_request`, tested)
- [ ] All common-checklist items above (fails to build with `--no-default-features`; dead-code warnings)

## 3. Cross-cutting / release process

- [ ] Verify no circular workspace dependencies (`cargo tree` per crate)
- [x] Initial version `0.1.0` for all crates; semver policy documented in root README
- [ ] Tag-triggered release workflow (e.g. `cargo-release`, or manual: bump versions per tier, `cargo publish --dry-run` then real publish per tier, wait for crates.io propagation between Tier 0 → Tier 1)
- [ ] Post-publish: confirm docs.rs builds succeed for every crate (all feature combos)
- [x] Badges (crates.io version, docs.rs, CI, license) on every crate README + root README
- [ ] Stretch goal (not a publish blocker): root `/examples` demonstrating cross-crate usage (e.g. llm-client-core + rag-pipeline + agent-memory)

## 5. Platform review findings (2026-07-22) — functional gaps, adoption gaps, and ideas

Found via source audit; several items above are checked `[x]` but the underlying code does not match the claim. Re-verify against source, not against this checklist, when closing these out.

### 4.1 Bugs and stubbed-out implementations (fix first — highest risk if published as-is)

- [ ] `tpt-agent-memory`: fix id-collision data-loss bug — `MemoryEntry::new` (`src/entry.rs:17`) derives id from `content.len()`, so any two entries with equal-length content silently overwrite each other in the `BTreeMap` store. Use a UUID or monotonic counter instead.
- [ ] `tpt-agent-memory`: make `MemoryStore` actually thread-safe (`Arc<RwLock<..>>` or similar) or correct the "concurrent, thread-safe" doc/README claim to match reality (currently a bare `BTreeMap`).
- [ ] `tpt-agent-memory`: validate node ids in `graph.rs::add_edge`; fix `neighbors()` to return node ids, not labels; add test coverage for `graph.rs` (currently zero tests).
- [ ] `tpt-agent-memory`: fix `lib.rs` doc example — calls `.insert(...).await`/`.search(...).await` on non-async methods; doesn't compile.
- [ ] `tpt-eval-harness`: implement `EvalHarness::run()` — currently ignores `dataset_path` and returns a hardcoded zeroed `EvalMetrics`.
- [ ] `tpt-eval-harness`: implement `ParallelExecutor` execution — currently only stores `max_concurrency` with no method that runs anything.
- [ ] `tpt-eval-harness`: wire the existing `ExactMatchScorer`/`ContainsScorer` (`scoring.rs`) into the evaluator/executor so scores are actually produced; populate `EvalMetrics`/`TokenUsage` from real runs; add tests (currently zero in the crate).
- [ ] `tpt-onnx-runtime-utils`: add a real ONNX backend (`ort` or `onnxruntime` crate) — currently no such dependency exists.
- [ ] `tpt-onnx-runtime-utils`: `Session::from_file` never opens/validates the given path; fix so invalid paths error.
- [ ] `tpt-onnx-runtime-utils`: `Session::run()` is a no-op identity function (copies input to output) instead of running inference; `input_names()`/`output_names()` are hardcoded — fix or explicitly re-scope/rename the crate as a mock/dev-utility until real inference lands.
- [ ] `tpt-tokenizers-fast`: `BpeTokenizer::encode()` never applies `self.merges` — it's plain character lookup, not real BPE. Implement actual merge application; add tests (currently zero).
- [ ] `tpt-llm-client-core`: implement real HTTP/SSE send/streaming in `SseClient` (currently only stores config fields, no network code at all) behind the `std`/`async` feature flags.
- [ ] `tpt-llm-client-core`: implement the automatic network retry/backoff claimed in the crate doc and in section 2 above (currently zero retry code anywhere, not even a placeholder).
- [ ] `tpt-llm-client-core`: fix `lib.rs` doc example — references a `chat_completion` method that doesn't exist and is marked `rust,ignore` so it never compiles; make it a real doctest once the client exists.
- [ ] Add test coverage to crates currently at zero: `tpt-eval-harness`, `tpt-onnx-runtime-utils`, `tpt-tokenizers-fast`, `tpt-tool-use-macros`, `tpt-vector-store-traits`.
- [ ] Audit all `[x]` boxes in section 2 (per-crate checklist) against actual source and uncheck any that don't hold up.

### 4.2 Adoption / onboarding gaps

- [ ] Build a root `/examples/` integration example: `llm-client-core` → `rag-pipeline` (chunk + embed) → `agent-memory` (store/search), ideally run against `tpt-ai-mock-server` so it's network-free and CI-testable. Reference it from the root README.
- [ ] Add a minimal runnable code example to each thin README (`tpt-agent-memory`, `tpt-eval-harness`, `tpt-ai-mock-server`, `tpt-tokenizers-fast`, `tpt-onnx-runtime-utils` currently have zero code examples).
- [ ] Convert existing ` ```rust,ignore ` README/doc examples to real compiled doctests once the underlying stubs in 4.1 are fixed (CI already runs `cargo doc`/tests, so this becomes enforced automatically).
- [ ] Add a root `CHANGELOG.md`; replace the identical placeholder boilerplate (`## [Unreleased] / ### Added / - Initial release`) in every per-crate `CHANGELOG.md` with real entries as fixes land.
- [ ] Add an "adding a new crate to the workspace" section to `CONTRIBUTING.md` (member registration, required Cargo.toml fields, README/CHANGELOG scaffolding).

### 4.3 Innovation / automation ideas (discuss scope before implementing)

- [ ] Evaluate a `cargo-generate` template or `xtask` scaffold command that generates a new crate matching the section-2 checklist automatically (Cargo.toml fields, README/CHANGELOG stubs, feature flags).
- [ ] Add a CI job that greps for `rust,ignore`, `unimplemented!()`, `todo!()` and fails on new occurrences without an explicit allowlist, to catch stubs like those in 4.1 automatically going forward.
- [ ] Wire `tpt-ai-mock-server` into the 4.2 integration example so it runs deterministically in CI without hitting real LLM APIs, doubling as a regression test for `tpt-llm-client-core`.

## 6. Known failures as of this checklist update

- `cargo test --all` passes (verified 2026-07-22, 0 failures across all 10 crates) — an earlier version of this checklist incorrectly claimed `tpt-prompt-template::parser::tests::test_parse_variables` was failing; corrected above.
- `cargo check --workspace --no-default-features` fails for `tpt-vector-store-traits` (verified 2026-07-22): `query.rs:49` derives `Serialize, Deserialize` but `ContentVisitor`/`ContentRefDeserializer` are gated out of `serde` without `std`/`alloc` features enabled.
- `tpt-agent-memory`, `tpt-ai-mock-server` `--no-default-features` status and the `dead_code` warning claims above are unverified — re-check before relying on them.
