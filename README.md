# Rust Benchmarks

A comprehensive and well-structured Rust benchmarking project using Criterion.

## Features

- **Sorting algorithms** (bubble sort, quick sort, merge sort)
- **Parsing operations** (integer parsing, CSV parsing)
- **Algorithm implementations** (fibonacci, search, prime sieve, GCD, matrix multiplication)
- **Data structure operations** (HashMap, BTreeMap, HashSet, Vec, VecDeque, BinaryHeap)
- **String operations** (concatenation, manipulation, search, case conversion)
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
├── lib.rs              # Main library exports
├── sorting.rs          # Sorting algorithms
├── parsing.rs          # Parsing functions
├── algorithms.rs       # Algorithm implementations
├── data_structures.rs  # Data structure operations
├── string_operations.rs # String manipulation
└── bin/
    └── benchmark.rs    # Manual benchmark runner
benches/
└── benchmarks.rs       # Criterion benchmarks
```

## Benchmark Categories

### Sorting
- Bubble Sort (O(n²))
- Quick Sort (O(n log n))
- Merge Sort (O(n log n))

### Parsing
- Integer parsing (simple vs validation)
- CSV parsing (simple vs with quotes)

### Algorithms
- Fibonacci (recursive, iterative, dynamic programming)
- Search algorithms (binary vs linear)
- Prime number generation (Sieve of Eratosthenes)
- Greatest Common Divisor (Euclidean vs naive)
- Matrix multiplication

### Data Structures
- Creation performance (HashMap, BTreeMap, HashSet, Vec, VecDeque)
- Lookup/search performance comparisons
- Memory allocation patterns

### String Operations
- Concatenation methods (+, push_str, format, collect)
- String manipulation (reverse, split, replace)
- Case conversion (uppercase, lowercase)
- String search (contains)

## Adding New Benchmarks

1. Add your benchmark functions to the appropriate module in `src/`
2. Export them in `src/lib.rs`
3. Add benchmark cases to `benches/benchmarks.rs`
4. Optionally add manual benchmarks to `src/bin/benchmark.rs`

## Dependencies

- `criterion`: Statistical benchmarking framework
- Standard library only for core functionality

## Performance Insights

This project demonstrates:
- Algorithm complexity differences
- Data structure trade-offs
- String operation efficiency
- Memory allocation patterns
- Built-in vs custom implementations 