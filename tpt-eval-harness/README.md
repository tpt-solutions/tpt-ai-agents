# tpt-eval-harness

[![crates.io](https://img.shields.io/crates/v/tpt-eval-harness.svg)](https://crates.io/crates/tpt-eval-harness)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

Tools for running LLM eval datasets deterministically with parallel execution.

**When to use this crate:** you have a fixed set of (prompt, expected
answer) pairs and want to score a model against them repeatably — for
regression testing a prompt change, comparing models, or CI gating on
accuracy.

## Features

- `std` (default): Enables standard library features, including loading
  datasets from a JSONL file (`EvalHarness::run_file`)
- Parallel execution engine (`ParallelExecutor`)
- Token usage and latency tracking

## Usage

```rust
use tpt_eval_harness::{EvalHarness, EvalConfig, EvalSample};

let harness = EvalHarness::new(EvalConfig::default());
let samples = vec![
    EvalSample::new("What is 2+2?", "4", "4"),
    EvalSample::new("Capital of France?", "Paris", "Lyon"),
];
let metrics = harness.run(&samples);
assert_eq!(metrics.correct, 1);
```

Or from a JSONL file (one JSON-encoded `EvalSample` per line), with `std`:

```rust,no_run
use tpt_eval_harness::{EvalHarness, EvalConfig};

let harness = EvalHarness::new(EvalConfig::default());
let metrics = harness.run_file("eval_dataset.jsonl").unwrap();
println!("accuracy: {}", metrics.accuracy());
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
