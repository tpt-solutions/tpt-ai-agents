# tpt-onnx-runtime-utils

[![crates.io](https://img.shields.io/crates/v/tpt-onnx-runtime-utils.svg)](https://crates.io/crates/tpt-onnx-runtime-utils)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../LICENSE-MIT)

High-level wrappers for local ONNX inference, powered by the pure-Rust
[`tract-onnx`](https://docs.rs/tract-onnx) engine — no system ONNX Runtime
installation or C toolchain required.

**When to use this crate:** you want to run a small local model (e.g. an
embedding model or classifier) without a hosted API call, and don't want to
deal with the C++ ONNX Runtime toolchain.

## Features

- `std` (default): Enables real ONNX inference via `tract-onnx`. Without this
  feature, `Session::from_file` only records the model path and `Session::run`
  returns an error, since `tract-onnx` requires the standard library.

## Usage

```rust,no_run
use tpt_onnx_runtime_utils::{Session, Tensor};

let session = Session::from_file("model.onnx").unwrap();
let input = Tensor::from_slice(&[1.0, 2.0, 3.0], &[1, 3]).unwrap();
let _output = session.run(&input).unwrap();
```

## License

Dual-licensed under [MIT](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE).
