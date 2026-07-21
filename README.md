# tpt-ai-agents

[![CI](https://github.com/tpt/tpt-ai-agents/actions/workflows/ci.yml/badge.svg)](https://github.com/tpt/tpt-ai-agents/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

Workspace of independent, composable Rust crates for LLM/agent infrastructure.

## Crates

| Crate | Version | Description | Tier |
|-------|---------|-------------|------|
| `tpt-llm-client-core` | [![crates.io](https://img.shields.io/crates/v/tpt-llm-client-core.svg)](https://crates.io/crates/tpt-llm-client-core) | Unified streaming HTTP/SSE client for OpenAI, Anthropic, Ollama | 0 |
| `tpt-vector-store-traits` | [![crates.io](https://img.shields.io/crates/v/tpt-vector-store-traits.svg)](https://crates.io/crates/tpt-vector-store-traits) | Standardized async traits for Qdrant, Milvus, pgvector | 0 |
| `tpt-rag-pipeline` | [![crates.io](https://img.shields.io/crates/v/tpt-rag-pipeline.svg)](https://crates.io/crates/tpt-rag-pipeline) | Chunking, embedding batching, context-window management | 1 |
| `tpt-tool-use-macros` | [![crates.io](https://img.shields.io/crates/v/tpt-tool-use-macros.svg)](https://crates.io/crates/tpt-tool-use-macros) | Proc macros exposing Rust functions as LLM tools | 0 |
| `tpt-prompt-template` | [![crates.io](https://img.shields.io/crates/v/tpt-prompt-template.svg)](https://crates.io/crates/tpt-prompt-template) | Safe, typed templating for prompts | 0 |
| `tpt-onnx-runtime-utils` | [![crates.io](https://img.shields.io/crates/v/tpt-onnx-runtime-utils.svg)](https://crates.io/crates/tpt-onnx-runtime-utils) | High-level wrappers for local ONNX inference | 0 |
| `tpt-tokenizers-fast` | [![crates.io](https://img.shields.io/crates/v/tpt-tokenizers-fast.svg)](https://crates.io/crates/tpt-tokenizers-fast) | Lightweight, no_std-compatible BPE/SentencePiece tokenization | 0 |
| `tpt-agent-memory` | [![crates.io](https://img.shields.io/crates/v/tpt-agent-memory.svg)](https://crates.io/crates/tpt-agent-memory) | In-memory and persistent graph/vector memory for multi-turn state | 1 |
| `tpt-eval-harness` | [![crates.io](https://img.shields.io/crates/v/tpt-eval-harness.svg)](https://crates.io/crates/tpt-eval-harness) | Tools for running LLM eval datasets deterministically | 1 |
| `tpt-ai-mock-server` | [![crates.io](https://img.shields.io/crates/v/tpt-ai-mock-server.svg)](https://crates.io/crates/tpt-ai-mock-server) | Mock server for testing LLM integrations | 0 |

## Publish Order

Publish by tier, waiting for crates.io propagation between tiers:

**Tier 0** (no internal deps): `tpt-tokenizers-fast`, `tpt-llm-client-core`, `tpt-vector-store-traits`, `tpt-prompt-template`, `tpt-tool-use-macros`, `tpt-onnx-runtime-utils`, `tpt-ai-mock-server`

**Tier 1** (depend on Tier 0): `tpt-rag-pipeline`, `tpt-agent-memory`, `tpt-eval-harness`

## MSRV

Rust **1.75** — enforced via `rust-toolchain.toml` and CI.

## License

Dual-licensed under [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE).
