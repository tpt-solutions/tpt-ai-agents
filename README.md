# tpt-ai-agents

[![CI](https://github.com/tpt-solutions/tpt-ai-agents/actions/workflows/ci.yml/badge.svg)](https://github.com/tpt-solutions/tpt-ai-agents/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/tpt-solutions/tpt-ai-agents/branch/main/graph/badge.svg)](https://codecov.io/gh/tpt-solutions/tpt-ai-agents)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

Workspace of independent, composable Rust crates for LLM/agent infrastructure.

See [GETTING_STARTED.md](GETTING_STARTED.md) for a quickstart and
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for how the crates compose.

## Crates

| Crate | Version | Maturity | Description | Tier |
|-------|---------|----------|-------------|------|
| `tpt-llm-client-core` | [![crates.io](https://img.shields.io/crates/v/tpt-llm-client-core.svg)](https://crates.io/crates/tpt-llm-client-core) | **Production** | Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama with automatic retry/backoff | 0 |
| `tpt-vector-store-traits` | [![crates.io](https://img.shields.io/crates/v/tpt-vector-store-traits.svg)](https://crates.io/crates/tpt-vector-store-traits) | **Beta** | Standardized async traits for Qdrant, Milvus, pgvector | 0 |
| `tpt-rag-pipeline` | [![crates.io](https://img.shields.io/crates/v/tpt-rag-pipeline.svg)](https://crates.io/crates/tpt-rag-pipeline) | **Beta** | Chunking, embedding batching, context-window management | 1 |
| `tpt-tool-use-macros` | [![crates.io](https://img.shields.io/crates/v/tpt-tool-use-macros.svg)](https://crates.io/crates/tpt-tool-use-macros) | **Production** | Proc macros exposing Rust functions as LLM tools | 0 |
| `tpt-prompt-template` | [![crates.io](https://img.shields.io/crates/v/tpt-prompt-template.svg)](https://crates.io/crates/tpt-prompt-template) | **Production** | Safe, typed templating for prompts | 0 |
| `tpt-onnx-runtime-utils` | [![crates.io](https://img.shields.io/crates/v/tpt-onnx-runtime-utils.svg)](https://crates.io/crates/tpt-onnx-runtime-utils) | **Beta** | High-level wrappers for local ONNX inference | 0 |
| `tpt-tokenizers-fast` | [![crates.io](https://img.shields.io/crates/v/tpt-tokenizers-fast.svg)](https://crates.io/crates/tpt-tokenizers-fast) | **Production** | Lightweight, no_std-compatible BPE/SentencePiece tokenization | 0 |
| `tpt-agent-memory` | [![crates.io](https://img.shields.io/crates/v/tpt-agent-memory.svg)](https://crates.io/crates/tpt-agent-memory) | **Production** | In-memory and file-persistent graph memory with keyword and embedding-based semantic search for multi-turn state | 1 |
| `tpt-eval-harness` | [![crates.io](https://img.shields.io/crates/v/tpt-eval-harness.svg)](https://crates.io/crates/tpt-eval-harness) | **Beta** | Tools for running LLM eval datasets deterministically | 1 |
| `tpt-ai-mock-server` | [![crates.io](https://img.shields.io/crates/v/tpt-ai-mock-server.svg)](https://crates.io/crates/tpt-ai-mock-server) | **Production** | Mock server for testing LLM integrations | 0 |

**Maturity levels:**
- **Production** — full implementation, comprehensive tests, real-world usage expected
- **Beta** — functional implementation, may have API adjustments before 1.0

## Publish Order

Publish by tier, waiting for crates.io propagation between tiers:

**Tier 0** (no internal deps): `tpt-tokenizers-fast`, `tpt-llm-client-core`, `tpt-vector-store-traits`, `tpt-prompt-template`, `tpt-tool-use-macros`, `tpt-onnx-runtime-utils`, `tpt-ai-mock-server`

**Tier 1** (depend on Tier 0): `tpt-rag-pipeline`, `tpt-agent-memory`, `tpt-eval-harness`

## MSRV

Rust **1.75** — enforced via `rust-toolchain.toml` and CI.

## License

Dual-licensed under [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE).
