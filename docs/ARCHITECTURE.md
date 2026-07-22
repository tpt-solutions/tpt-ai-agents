# Architecture

## Tier system

The workspace is split into two publish tiers, reflecting internal
dependencies (see the root [README.md](../README.md) for the full list):

- **Tier 0** — no internal dependencies. Each can be adopted completely
  independently: `tpt-llm-client-core`, `tpt-vector-store-traits`,
  `tpt-prompt-template`, `tpt-tool-use-macros`, `tpt-onnx-runtime-utils`,
  `tpt-tokenizers-fast`, `tpt-ai-mock-server`.
- **Tier 1** — compose Tier 0 crates via optional dependencies:
  `tpt-rag-pipeline` (optionally uses `tpt-tokenizers-fast`,
  `tpt-vector-store-traits`, `tpt-llm-client-core` behind its `full`
  feature), `tpt-agent-memory` (optionally uses `tpt-vector-store-traits`),
  `tpt-eval-harness` (optionally uses `tpt-llm-client-core`).

Tiers exist for the crates.io publish order (Tier 0 must land first so Tier
1's path dependencies resolve), not for a strict layering rule — a Tier 0
crate never depends on anything in Tier 1.

## How the crates compose in an agent loop

A typical agent turn touches several crates, each replaceable independently:

```
   user input
       |
       v
[tpt-rag-pipeline]  chunk & retrieve relevant context
       |
       v
[tpt-agent-memory]  store/search conversation + retrieved state
       |
       v
[tpt-prompt-template]  render the prompt with that context
       |
       v
[tpt-llm-client-core]  call the model (retries on transient failure)
       |
       v
   model replies, optionally requesting a tool call
       |
       v
[tpt-tool-use-macros]  #[tool]-generated dispatcher runs the Rust function
       |
       v
[tpt-agent-memory]  store the tool result, loop back to the model
```

`tpt-ai-mock-server` sits alongside this loop, not in it — it replays
recorded responses so the same client code can be tested offline. See
[`examples/full_agent_loop.rs`](../examples/full_agent_loop.rs) for a
working version of this exact flow, and
[`examples/rag_memory.rs`](../examples/rag_memory.rs) for just the
chunk-and-retrieve piece.

`tpt-tokenizers-fast` and `tpt-onnx-runtime-utils` are used off this main
path — for counting/limiting tokens before a request, or for running a
local model instead of a hosted one. `tpt-vector-store-traits` defines the
async traits `tpt-rag-pipeline`/`tpt-agent-memory` would use for a real
vector database backend; no concrete adapter (Qdrant, pgvector, etc.) ships
yet, so today's semantic search in `tpt-agent-memory` is a self-contained
cosine-similarity scan over in-memory embeddings, not a vector DB query.
`tpt-eval-harness` runs outside the live loop entirely — it replays a fixed
dataset through `tpt-llm-client-core` to score a model deterministically.

## `no_std` / feature-flag philosophy

Every crate is `#![no_std]` by default with `extern crate alloc;`, gated by
two conventional features:

- `std` (in `default`): enables anything requiring the standard library —
  networking (`reqwest`), file I/O, threading primitives. Without it, a
  crate still builds and its core data structures/algorithms still work; it
  just can't do I/O.
- `async` (alias for `std`): kept as a separate name so call sites can
  express "I need async" without implying "I need every std feature,"
  even though today it maps 1:1 onto `std`.

This lets a crate like `tpt-tokenizers-fast` or `tpt-prompt-template` be
used in an embedded or WASM context with zero std dependency, while crates
that inherently need the network (`tpt-llm-client-core`) or the filesystem
(`tpt-agent-memory`'s persistence) gate that functionality behind `std`
rather than requiring it unconditionally. CI's `no-std-check` job
(`.github/workflows/ci.yml`) verifies every crate still builds with
`--no-default-features`.

See [CONTRIBUTING.md](../CONTRIBUTING.md) for the checklist a new crate
should follow to fit this pattern.
