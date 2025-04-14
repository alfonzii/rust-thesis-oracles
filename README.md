# DLC Fast Prototyping

**DLC Fast Prototyping** is a lightweight framework for fast experimentation, benchmarking, and development of [Discreet Log Contracts (DLCs)](https://adiabat.github.io/dlc.pdf) on Bitcoin.  
It allows testing various cryptographic configurations—such as ECDSA vs. Schnorr adaptor signatures—and running detailed benchmarks for performance analysis.


## Prerequisites

- Rust (recommended version: `cargo 1.85.0`)
- A Unix-like system is recommended (Linux/macOS)

Ensure Rust is installed via [rustup](https://rustup.rs/):

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


## Running the Program
### Run with Default Configuration
To run the program with the default settings (ECDSA + serial execution):
```
cargo run
```
This will execute the DLC setup using serial anticipation point computation and ECDSA adaptor signatures.

### Run with Custom Features
To test specific configurations (e.g., using Schnorr and enabling parallelism), disable default features and specify the desired ones:
```
cargo run --release --no-default-features --features "enable-benchmarks, schnorr, parallel-cpt"
```
- `--release` enables compiler optimizations for realistic performance.
- Default feature is `ecdsa`; use `--no-default-features` to exclude it.


## Additional Configuration
Parameters that are not controlled via feature flags (e.g., number of outcomes, anticipation point computation method, etc.) are located in:
```
src/config.rs
```
This file is thoroughly commented to make customization easy.


## Benchmarks
The framework supports both:
- **End-to-end DLC benchmarks** (via `enable-benchmarks` feature)
- **Function-specific math microbenchmarks** (via `math-bench.rs`)

### Run High-Level Benchmarks
To measure overall runtime of DLC creation and execution:
```
cargo run --release --features "enable-benchmarks"
```
Two key runtime bottlenecks typically emerge:

- `Init storage`

- `Verify adaptors`

These are analyzed further using fine-grained function-level benchmarking.

### Run Microbenchmarks (Function-level)

To benchmark individual functions in isolation:
```
cargo bench --bench math-bench --no-default-features --features "schnorr"
```
This allows testing core cryptographic primitives.

> Only ecdsa or schnorr features are relevant for function-level benchmarks.

### Run Comparative Decision Benchmarks

During development, several alternative designs were benchmarked to guide decisions. These comparisons are available in:
```
benches/benchmark.rs
```
You can run them similarly:
```
cargo bench --bench benchmark --no-default-features --features "schnorr"
```

## About

This framework was developed as part of the thesis project _“Practical Oracle-Based Bitcoin Payments”_.
The goal was to create a flexible, modular environment for evaluating and optimizing DLC protocol components—particularly focusing on cryptographic performance and protocol design.

For more details on the design rationale and performance analysis, please refer to the [thesis](https://google.com).

## License

This project is licensed under the [MIT License](LICENSE).

Parts of the code are adapted from MIT-licensed projects, including:
- [`rust-dlc`](https://github.com/p2pderivatives/rust-dlc)

See [NOTICE](NOTICE) for full attribution details.