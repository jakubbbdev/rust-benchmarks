# Rust Benchmarks

A clean and well-structured Rust benchmarking project using Criterion.

## Features

- Sorting algorithm benchmarks (bubble sort, quick sort, merge sort)
- Parsing benchmarks (integer parsing, CSV parsing)
- Modular structure for easy extension
- Criterion-based benchmarking with detailed statistics
- Manual benchmark runner for quick testing

## Running Benchmarks

### Using Criterion (Recommended)

Run all benchmarks with detailed statistics:

```bash
cargo bench
```

### Manual Benchmarks

Run quick manual benchmarks:

```bash
cargo run --bin benchmark
```

## Project Structure

```
src/
├── lib.rs          # Main library exports
├── sorting.rs      # Sorting algorithms
├── parsing.rs      # Parsing functions
└── bin/
    └── benchmark.rs # Manual benchmark runner
benches/
└── benchmarks.rs   # Criterion benchmarks
```

## Adding New Benchmarks

1. Add your benchmark functions to the appropriate module in `src/`
2. Export them in `src/lib.rs`
3. Add benchmark cases to `benches/benchmarks.rs`
4. Optionally add manual benchmarks to `src/bin/benchmark.rs`

## Dependencies

- `criterion`: Statistical benchmarking framework
- Standard library only for core functionality 