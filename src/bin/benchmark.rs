use rust_benchmarks::{
    bubble_sort, generate_csv_data, generate_random_array, generate_test_data, merge_sort,
    parse_csv_simple, parse_csv_with_quotes, parse_integers_simple, parse_integers_with_validation,
    quick_sort,
    // Algorithms
    fibonacci_recursive, fibonacci_iterative, fibonacci_dynamic, binary_search, linear_search,
    prime_sieve, gcd_euclidean, gcd_naive, matrix_multiply_naive, generate_matrix,
    // Data structures
    hash_map_operations, btree_map_operations, hash_set_operations, btree_set_operations,
    vec_operations, vec_deque_operations, binary_heap_operations, linked_list_operations,
    stack_operations, queue_operations, hash_map_lookup, btree_map_lookup, vec_search,
    hash_set_contains, generate_search_keys,
    // String operations
    string_concat_plus, string_concat_push_str, string_concat_format, string_concat_collect,
    string_reverse_naive, string_reverse_bytes, string_split_whitespace, string_split_char,
    string_replace_naive, string_replace_builtin, string_uppercase_naive, string_uppercase_builtin,
    string_lowercase_naive, string_lowercase_builtin, string_trim_naive, string_trim_builtin,
    generate_test_string, generate_large_string, string_contains_naive, string_contains_builtin,
};
use std::time::Instant;

fn main() {
    println!("Running manual benchmarks...\n");
    
    run_sorting_benchmarks();
    println!();
    run_parsing_benchmarks();
    println!();
    run_algorithms_benchmarks();
    println!();
    run_data_structures_benchmarks();
    println!();
    run_string_operations_benchmarks();
}

fn run_sorting_benchmarks() {
    println!("Sorting Benchmarks:");
    println!("==================");
    
    let sizes = [100, 1000, 10000];
    
    for &size in &sizes {
        let array = generate_random_array(size);
        println!("\nArray size: {}", size);
        
        let start = Instant::now();
        let _result = bubble_sort(&array);
        let duration = start.elapsed();
        println!("  Bubble sort: {:?}", duration);
        
        let start = Instant::now();
        let _result = quick_sort(&array);
        let duration = start.elapsed();
        println!("  Quick sort:  {:?}", duration);
        
        let start = Instant::now();
        let _result = merge_sort(&array);
        let duration = start.elapsed();
        println!("  Merge sort:  {:?}", duration);
    }
}

fn run_parsing_benchmarks() {
    println!("Parsing Benchmarks:");
    println!("==================");
    
    let sizes = [100, 1000, 10000];
    
    for &size in &sizes {
        let data = generate_test_data(size);
        let csv_data = generate_csv_data(size, 5);
        
        println!("\nData size: {}", size);
        
        let start = Instant::now();
        let _result = parse_integers_simple(&data);
        let duration = start.elapsed();
        println!("  Integer parsing (simple):     {:?}", duration);
        
        let start = Instant::now();
        let _result = parse_integers_with_validation(&data);
        let duration = start.elapsed();
        println!("  Integer parsing (validation): {:?}", duration);
        
        let start = Instant::now();
        let _result = parse_csv_simple(&csv_data);
        let duration = start.elapsed();
        println!("  CSV parsing (simple):         {:?}", duration);
        
        let start = Instant::now();
        let _result = parse_csv_with_quotes(&csv_data);
        let duration = start.elapsed();
        println!("  CSV parsing (with quotes):    {:?}", duration);
    }
}

fn run_algorithms_benchmarks() {
    println!("Algorithms Benchmarks:");
    println!("=====================");
    
    println!("\nFibonacci (n=30):");
    let start = Instant::now();
    let _result = fibonacci_recursive(30);
    let duration = start.elapsed();
    println!("  Recursive: {:?}", duration);
    
    println!("\nFibonacci (n=50):");
    let start = Instant::now();
    let _result = fibonacci_iterative(50);
    let duration = start.elapsed();
    println!("  Iterative: {:?}", duration);
    
    let start = Instant::now();
    let _result = fibonacci_dynamic(50);
    let duration = start.elapsed();
    println!("  Dynamic:   {:?}", duration);
    
    println!("\nSearch Algorithms:");
    let sorted_array = {
        let mut arr = generate_random_array(10000);
        arr.sort();
        arr
    };
    
    let start = Instant::now();
    let _result = binary_search(&sorted_array, &5000);
    let duration = start.elapsed();
    println!("  Binary search: {:?}", duration);
    
    let start = Instant::now();
    let _result = linear_search(&sorted_array, &5000);
    let duration = start.elapsed();
    println!("  Linear search: {:?}", duration);
    
    println!("\nPrime Sieve (n=10000):");
    let start = Instant::now();
    let _result = prime_sieve(10000);
    let duration = start.elapsed();
    println!("  Sieve: {:?}", duration);
    
    println!("\nGCD (123456, 789012):");
    let start = Instant::now();
    let _result = gcd_euclidean(123456, 789012);
    let duration = start.elapsed();
    println!("  Euclidean: {:?}", duration);
    
    let start = Instant::now();
    let _result = gcd_naive(123456, 789012);
    let duration = start.elapsed();
    println!("  Naive:     {:?}", duration);
    
    println!("\nMatrix Multiplication (50x50):");
    let matrix_a = generate_matrix(50);
    let matrix_b = generate_matrix(50);
    let start = Instant::now();
    let _result = matrix_multiply_naive(&matrix_a, &matrix_b);
    let duration = start.elapsed();
    println!("  Naive: {:?}", duration);
}

fn run_data_structures_benchmarks() {
    println!("Data Structures Benchmarks:");
    println!("==========================");
    
    let sizes = [100, 1000, 10000];
    
    for &size in &sizes {
        println!("\nCreation (size: {}):", size);
        
        let start = Instant::now();
        let _result = hash_map_operations(size);
        let duration = start.elapsed();
        println!("  HashMap:   {:?}", duration);
        
        let start = Instant::now();
        let _result = btree_map_operations(size);
        let duration = start.elapsed();
        println!("  BTreeMap:  {:?}", duration);
        
        let start = Instant::now();
        let _result = vec_operations(size);
        let duration = start.elapsed();
        println!("  Vec:       {:?}", duration);
        
        let start = Instant::now();
        let _result = hash_set_operations(size);
        let duration = start.elapsed();
        println!("  HashSet:   {:?}", duration);
    }
    
    println!("\nLookup Operations (1000 elements, 100 searches):");
    let hash_map = hash_map_operations(1000);
    let btree_map = btree_map_operations(1000);
    let vec = vec_operations(1000);
    let hash_set = hash_set_operations(1000);
    let search_keys = generate_search_keys(100);
    
    let start = Instant::now();
    let _result = hash_map_lookup(&hash_map, &search_keys);
    let duration = start.elapsed();
    println!("  HashMap lookup: {:?}", duration);
    
    let start = Instant::now();
    let _result = btree_map_lookup(&btree_map, &search_keys);
    let duration = start.elapsed();
    println!("  BTreeMap lookup: {:?}", duration);
    
    let start = Instant::now();
    let _result = vec_search(&vec, &search_keys);
    let duration = start.elapsed();
    println!("  Vec search: {:?}", duration);
    
    let start = Instant::now();
    let _result = hash_set_contains(&hash_set, &search_keys);
    let duration = start.elapsed();
    println!("  HashSet contains: {:?}", duration);
}

fn run_string_operations_benchmarks() {
    println!("String Operations Benchmarks:");
    println!("============================");
    
    println!("\nString Concatenation (100 words):");
    let start = Instant::now();
    let _result = string_concat_plus(100);
    let duration = start.elapsed();
    println!("  Plus operator: {:?}", duration);
    
    let start = Instant::now();
    let _result = string_concat_push_str(100);
    let duration = start.elapsed();
    println!("  Push_str:      {:?}", duration);
    
    let start = Instant::now();
    let _result = string_concat_format(100);
    let duration = start.elapsed();
    println!("  Format:        {:?}", duration);
    
    let start = Instant::now();
    let _result = string_concat_collect(100);
    let duration = start.elapsed();
    println!("  Collect:       {:?}", duration);
    
    println!("\nString Manipulation:");
    let test_string = generate_test_string(1000);
    let large_string = generate_large_string(100);
    
    let start = Instant::now();
    let _result = string_reverse_naive(&test_string);
    let duration = start.elapsed();
    println!("  Reverse (naive): {:?}", duration);
    
    let start = Instant::now();
    let _result = string_reverse_bytes(&test_string);
    let duration = start.elapsed();
    println!("  Reverse (bytes): {:?}", duration);
    
    let start = Instant::now();
    let _result = string_split_whitespace(&test_string);
    let duration = start.elapsed();
    println!("  Split whitespace: {:?}", duration);
    
    let start = Instant::now();
    let _result = string_replace_naive(&test_string, "word", "term");
    let duration = start.elapsed();
    println!("  Replace (naive): {:?}", duration);
    
    let start = Instant::now();
    let _result = string_replace_builtin(&test_string, "word", "term");
    let duration = start.elapsed();
    println!("  Replace (builtin): {:?}", duration);
    
    let start = Instant::now();
    let _result = string_uppercase_naive(&large_string);
    let duration = start.elapsed();
    println!("  Uppercase (naive): {:?}", duration);
    
    let start = Instant::now();
    let _result = string_uppercase_builtin(&large_string);
    let duration = start.elapsed();
    println!("  Uppercase (builtin): {:?}", duration);
    
    let start = Instant::now();
    let _result = string_contains_naive(&large_string, "test");
    let duration = start.elapsed();
    println!("  Contains (naive): {:?}", duration);
    
    let start = Instant::now();
    let _result = string_contains_builtin(&large_string, "test");
    let duration = start.elapsed();
    println!("  Contains (builtin): {:?}", duration);
} 