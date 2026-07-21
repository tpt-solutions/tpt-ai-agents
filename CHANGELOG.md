# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace of 10 crates for LLM/agent infrastructure
- `tpt-llm-client-core`: SSE parser, streaming client stubs
- `tpt-vector-store-traits`: Async vector store traits with GATs
- `tpt-rag-pipeline`: Chunker with overlapping windows
- `tpt-tool-use-macros`: Proc macros for JSON schema generation
- `tpt-prompt-template`: Compile-time template validation
- `tpt-onnx-runtime-utils`: Tensor and session abstractions (mock backend)
- `tpt-tokenizers-fast`: BPE tokenizer with no_std support
- `tpt-agent-memory`: Memory store, graph, temporal decay, search
- `tpt-eval-harness`: Evaluation harness with scorer trait
- `tpt-ai-mock-server`: Mock LLM server with request validation
- CI pipeline: fmt, clippy, test matrix, no_std check, doc check, deny check
- Root `README.md`, `CONTRIBUTING.md`, licenses
- Integration example: rag-pipeline + agent-memory

### Fixed
- Agent memory id-collision bug (entries with equal-length content no longer overwrite)
- Agent memory `neighbors()` now returns node IDs, not labels
- Agent memory `add_edge` validates node existence
- `--no-default-features` builds for all crates (serde alloc, decay no_std, mock-server std gating)
- Doc examples now compile and pass as doctests
