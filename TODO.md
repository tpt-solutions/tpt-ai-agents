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
- [x] Confirm all 10 crate names are unclaimed on crates.io before first
      publish (2026-07-23, checked `crates.io/api/v1/crates/<name>` — all
      10 return 404). Ran `cargo publish --dry-run` for every crate too:
      found the README's "Tier 0 = no internal deps" claim was wrong for
      `tpt-llm-client-core`, which optionally depends on
      `tpt-prompt-template` for its `tool-use` feature — dry-run fails
      until that specific dependency is published first, regardless of
      "tier." `release.yml`'s actual publish order already has this right
      (`tpt-prompt-template` before `tpt-llm-client-core`); only the
      README's prose was misleading. Fixed.

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
- [x] Automatic network retries (exponential backoff via `RetryConfig`, 2026-07-22)
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
- [x] Automatic serialization/deserialization of tool arguments (see §4.1 — was implemented but left unchecked here; reconciled 2026-07-22)
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
- [x] Memory store with keyword (substring) search always available, plus
      real cosine-similarity embedding search via `SearchQuery::with_embedding`
      / `MemoryEntry::with_embedding` (2026-07-22) — previously the doc
      claimed "semantic search" but the implementation was substring-only
- [x] File-based persistence: `MemoryStore::save_to_file`/`load_from_file`
      (JSON, `std`-gated, 2026-07-22) — previously claimed but unimplemented
- [x] `MemoryEntry` unique-ID counter now uses `AtomicU64` instead of an
      unsynchronized `static mut` (2026-07-22) — the old version was a data
      race when entries were created from multiple threads via
      `ConcurrentMemoryStore`
- [x] Graph with validated edges and neighbor lookup (4 tests)
- [x] Temporal decay (1 test)
- [x] Unique ID generation (2 tests)

### tpt-eval-harness
- [x] Evaluation harness with scorer trait (1 test + 1 doctest)
- [x] Parallel executor with batch processing (1 test)

### tpt-ai-mock-server
- [x] Request validation (2 tests)
- [x] **Real request serving** (2026-07-22) — `MockServer::start()` previously
      only bound a `TcpListener` and returned its address; nothing ever
      accepted a connection, so `Router::find`/`validate_request` were dead
      code and the crate could not actually serve a mock response to a real
      HTTP client despite being described as a "mock server for testing LLM
      integrations." Implemented a background accept loop that parses
      minimal HTTP/1.1 requests, validates them, and replies with either a
      JSON body or an SSE stream from the queued `RecordedResponse`.
      `Router::take` now serves queued responses per path in FIFO order
      (supports scripting multi-turn conversations). Verified end-to-end
      against a real `reqwest` client in `tests/serves_http.rs` (4 tests).

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
- [x] Confirm all 10 crate names are unclaimed on crates.io (see §1)
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

### 4.2 Future work (in progress — platform review 2026-07-22)

Adoption / docs:
- [x] `GETTING_STARTED.md` (root) — smallest-possible working quickstart
- [x] `docs/ARCHITECTURE.md` — tier system, crate composition, no_std philosophy
- [x] End-to-end example wiring `llm-client-core` + `tool-use-macros` +
      `agent-memory` + `rag-pipeline` together, served by `tpt-ai-mock-server`
      so it's CI-testable with no API key (`examples/full_agent_loop.rs`,
      2026-07-22). This also required fixing two real bugs found along the
      way: the root `examples/` directory was never wired into the build
      (root `Cargo.toml` was a pure virtual workspace with no `[package]`,
      so `cargo build --example` failed for the *existing* `rag_memory.rs`
      example too — fixed by adding a `publish = false` root package), and
      `tpt-ai-mock-server`'s `MockServer::start()` bound a `TcpListener` but
      never accepted a connection (see its own entry below).
- [x] Expand thin per-crate READMEs (runnable snippet + "when to use this crate")
- [x] `cargo-generate` template for new crate scaffolding (`template/`)
- [x] Audit and fix `rust,ignore` doctests (2026-07-22): all 5 that were
      `rust,ignore` are now real (`no_run` or fully executed) doctests —
      `tpt-llm-client-core`, `tpt-ai-mock-server`, `tpt-eval-harness`,
      `tpt-onnx-runtime-utils`, `tpt-tool-use-macros`. Found and fixed two
      more doc/impl mismatches along the way: `tpt-tokenizers-fast`'s doc
      and README referenced a `BpeTokenizer::from_file` that doesn't exist
      (real constructor is `BpeTokenizer::new(vocab, merges)`), and
      `tpt-llm-client-core`'s README used a `client.chat_completion(...)`
      method that doesn't exist (real API is `client.send(&request)` /
      `client.stream(&request)`).
- [x] Fixed `tpt-vector-store-traits` doc/impl overclaim: crate doc and
      description said "GATs ... without boxing," but `VectorStore` uses
      `#[async_trait]`, which boxes futures and does not use GATs. Doc and
      description corrected to describe what's actually there (2026-07-22).

CI / hygiene:
- [x] Cross-platform CI matrix (clippy/test/msrv jobs now run
      `[ubuntu-latest, windows-latest]`)
- [x] CI job to detect `rust,ignore`/`unimplemented!()`/`todo!()` regressions
- [x] Dependabot config for dependency updates (`.github/dependabot.yml`)
- [x] Code coverage reporting (`cargo-llvm-cov` + codecov)

Innovative / stretch:
- [x] Per-crate maturity indicators in root README
- [x] Reference vector-store adapter example (`examples/qdrant_adapter.rs`)
      for `tpt-vector-store-traits`
- [x] Wire a real vector-store backend into `tpt-agent-memory`'s embedding
      search (2026-07-23): added `VectorBackedMemoryStore<S: VectorStore>`
      behind a new `vector-store` feature (`tpt-agent-memory/src/vector_backed.rs`).
      It keeps `MemoryStore` as the local source of truth for entry content
      and delegates only ID+embedding upsert/search/delete to the backend,
      so it works with any `VectorStore` implementation regardless of its
      `Payload` type. Along the way, discovered and fixed two more gaps:
      the optional `tpt-vector-store-traits` dependency had no feature that
      actually enabled it (only weak `?/` feature forwarding, which is a
      no-op without something else turning the dep on first — so it could
      never be reached), and `MemoryStore` had no `remove` method needed to
      keep the local store and vector backend in sync. Verified against an
      in-memory `VectorStore` fake in 3 new tests (19 total in the crate);
      `examples/qdrant_adapter.rs` also had a stale `tpt_vector_store_traits::store::CollectionInfo`
      path (the module isn't public) that failed `cargo build --all-features`
      — fixed to use the re-exported `CollectionInfo`.
- [x] `examples/agent_cli.rs` — runnable chat loop over the agent-loop example
- [x] `criterion` benchmarks for `tpt-tokenizers-fast` and the SSE parser in
      `tpt-llm-client-core`
