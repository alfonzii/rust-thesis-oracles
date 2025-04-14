# Dlc-Fast-Prototyping
Lightweight framework for fast prototyping and benchmarking of Discreet Log Contracts (DLCs) in Bitcoin. It allows you to quickly test different configurations and features, such as ECDSA or Schnorr adaptor signatures, and run benchmarks to evaluate performance.

## Prerequisites
You need Rust installed on your device. Program was tested on `cargo 1.85.0` version.

## How to use
### Running the Program with Default Configuration
To run the program with the default configuration (serial execution and ECDSA), simply execute the following command in the root directory:
```
cargo run
```

### Running the Program with Custom Features
To customize the configuration, you can use the `--no-default-features` flag, to get rid of ecdsa feature, which is the default flag for our project and specify the desired features. For example, to enable benchmarks, use Schnorr signatures, and enable parallel computation of anticipation points, run:
```
cargo run --release --no-default-features --features "enable-benchmarks, schnorr, parallel-cpt"
```
This disables the default ECDSA feature and enables the specified features. We also use `--release` for better performance.

### Additional Configuration
Some parameters cannot be changed via features and are defined in the `config.rs` file. This file is thoroughly commented to help you understand and modify these parameters as needed.

## Benchmarks
If you run program just as is, or with `enable-benchmarks` feature, you will get runtime benchmark of the whole DLC setup and execution. However, at standard situation, there are two functions that stick out, and those are **Init storage** and **Verify adaptors**. As they are more complex, we need to break them down more precisely to see, what takes how long. For that, we have specific benchmarks of functions that those two consist of, and it is in `math-bench.rs`.

For those benchmarks, it's irrelevant if we use any additional features other than `ecdsa` or `schnorr`, as we are benchmarking just one function, not whole run. For example, to run with schnorr, we can run it like:
```
cargo bench --bench math-bench --no-default-features --features "schnorr"
```

While developing this framework, we encountered several decision problems. To decide which way to go, we sometimes had to benchmark existing solutions or multiple options and we then chose, based on our criteria, which suits best. All of those are in `benchmark.rs` file. It can be also run, like:
```
cargo bench --bench math-bench --no-default-features --features "schnorr"
```