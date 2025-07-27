use criterion::{black_box, criterion_group, criterion_main, Criterion};
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

fn sorting_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");
    
    let small_array = generate_random_array(100);
    let medium_array = generate_random_array(1000);
    let large_array = generate_random_array(10000);
    
    group.bench_function("bubble_sort_100", |b| {
        b.iter(|| bubble_sort(black_box(&small_array)))
    });
    
    group.bench_function("quick_sort_100", |b| {
        b.iter(|| quick_sort(black_box(&small_array)))
    });
    
    group.bench_function("merge_sort_100", |b| {
        b.iter(|| merge_sort(black_box(&small_array)))
    });
    
    group.bench_function("quick_sort_1000", |b| {
        b.iter(|| quick_sort(black_box(&medium_array)))
    });
    
    group.bench_function("merge_sort_1000", |b| {
        b.iter(|| merge_sort(black_box(&medium_array)))
    });
    
    group.bench_function("quick_sort_10000", |b| {
        b.iter(|| quick_sort(black_box(&large_array)))
    });
    
    group.bench_function("merge_sort_10000", |b| {
        b.iter(|| merge_sort(black_box(&large_array)))
    });
    
    group.finish();
}

fn parsing_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    
    let small_data = generate_test_data(100);
    let medium_data = generate_test_data(1000);
    let large_data = generate_test_data(10000);
    
    let small_csv = generate_csv_data(100, 5);
    let medium_csv = generate_csv_data(1000, 5);
    let large_csv = generate_csv_data(10000, 5);
    
    group.bench_function("parse_integers_simple_100", |b| {
        b.iter(|| parse_integers_simple(black_box(&small_data)))
    });
    
    group.bench_function("parse_integers_validation_100", |b| {
        b.iter(|| parse_integers_with_validation(black_box(&small_data)))
    });
    
    group.bench_function("parse_integers_simple_1000", |b| {
        b.iter(|| parse_integers_simple(black_box(&medium_data)))
    });
    
    group.bench_function("parse_integers_validation_1000", |b| {
        b.iter(|| parse_integers_with_validation(black_box(&medium_data)))
    });
    
    group.bench_function("parse_csv_simple_100", |b| {
        b.iter(|| parse_csv_simple(black_box(&small_csv)))
    });
    
    group.bench_function("parse_csv_quotes_100", |b| {
        b.iter(|| parse_csv_with_quotes(black_box(&small_csv)))
    });
    
    group.bench_function("parse_csv_simple_1000", |b| {
        b.iter(|| parse_csv_simple(black_box(&medium_csv)))
    });
    
    group.bench_function("parse_csv_quotes_1000", |b| {
        b.iter(|| parse_csv_with_quotes(black_box(&medium_csv)))
    });
    
    group.finish();
}

fn algorithms_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithms");
    
    let sorted_array = {
        let mut arr = generate_random_array(10000);
        arr.sort();
        arr
    };
    let search_targets = vec![100, 5000, 9999];
    
    group.bench_function("fibonacci_recursive_30", |b| {
        b.iter(|| fibonacci_recursive(black_box(30)))
    });
    
    group.bench_function("fibonacci_iterative_1000", |b| {
        b.iter(|| fibonacci_iterative(black_box(1000)))
    });
    
    group.bench_function("fibonacci_dynamic_1000", |b| {
        b.iter(|| fibonacci_dynamic(black_box(1000)))
    });
    
    group.bench_function("binary_search_10000", |b| {
        b.iter(|| {
            for &target in &search_targets {
                binary_search(black_box(&sorted_array), black_box(&target));
            }
        })
    });
    
    group.bench_function("linear_search_10000", |b| {
        b.iter(|| {
            for &target in &search_targets {
                linear_search(black_box(&sorted_array), black_box(&target));
            }
        })
    });
    
    group.bench_function("prime_sieve_10000", |b| {
        b.iter(|| prime_sieve(black_box(10000)))
    });
    
    group.bench_function("gcd_euclidean", |b| {
        b.iter(|| gcd_euclidean(black_box(123456), black_box(789012)))
    });
    
    group.bench_function("gcd_naive", |b| {
        b.iter(|| gcd_naive(black_box(123456), black_box(789012)))
    });
    
    let matrix_a = generate_matrix(50);
    let matrix_b = generate_matrix(50);
    
    group.bench_function("matrix_multiply_50x50", |b| {
        b.iter(|| matrix_multiply_naive(black_box(&matrix_a), black_box(&matrix_b)))
    });
    
    group.finish();
}

fn data_structures_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structures");
    
    let sizes = [100, 1000, 10000];
    
    for &size in &sizes {
        group.bench_function(&format!("hash_map_create_{}", size), |b| {
            b.iter(|| hash_map_operations(black_box(size)))
        });
        
        group.bench_function(&format!("btree_map_create_{}", size), |b| {
            b.iter(|| btree_map_operations(black_box(size)))
        });
        
        group.bench_function(&format!("hash_set_create_{}", size), |b| {
            b.iter(|| hash_set_operations(black_box(size)))
        });
        
        group.bench_function(&format!("btree_set_create_{}", size), |b| {
            b.iter(|| btree_set_operations(black_box(size)))
        });
        
        group.bench_function(&format!("vec_create_{}", size), |b| {
            b.iter(|| vec_operations(black_box(size)))
        });
        
        group.bench_function(&format!("vec_deque_create_{}", size), |b| {
            b.iter(|| vec_deque_operations(black_box(size)))
        });
    }
    
    let hash_map = hash_map_operations(1000);
    let btree_map = btree_map_operations(1000);
    let vec = vec_operations(1000);
    let hash_set = hash_set_operations(1000);
    let search_keys = generate_search_keys(100);
    
    group.bench_function("hash_map_lookup_1000", |b| {
        b.iter(|| hash_map_lookup(black_box(&hash_map), black_box(&search_keys)))
    });
    
    group.bench_function("btree_map_lookup_1000", |b| {
        b.iter(|| btree_map_lookup(black_box(&btree_map), black_box(&search_keys)))
    });
    
    group.bench_function("vec_search_1000", |b| {
        b.iter(|| vec_search(black_box(&vec), black_box(&search_keys)))
    });
    
    group.bench_function("hash_set_contains_1000", |b| {
        b.iter(|| hash_set_contains(black_box(&hash_set), black_box(&search_keys)))
    });
    
    group.finish();
}

fn string_operations_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    
    let small_size = 100;
    let medium_size = 1000;
    let large_size = 10000;
    
    group.bench_function("string_concat_plus_100", |b| {
        b.iter(|| string_concat_plus(black_box(small_size)))
    });
    
    group.bench_function("string_concat_push_str_100", |b| {
        b.iter(|| string_concat_push_str(black_box(small_size)))
    });
    
    group.bench_function("string_concat_format_100", |b| {
        b.iter(|| string_concat_format(black_box(small_size)))
    });
    
    group.bench_function("string_concat_collect_100", |b| {
        b.iter(|| string_concat_collect(black_box(small_size)))
    });
    
    let test_string = generate_test_string(1000);
    let large_string = generate_large_string(100);
    
    group.bench_function("string_reverse_naive", |b| {
        b.iter(|| string_reverse_naive(black_box(&test_string)))
    });
    
    group.bench_function("string_reverse_bytes", |b| {
        b.iter(|| string_reverse_bytes(black_box(&test_string)))
    });
    
    group.bench_function("string_split_whitespace", |b| {
        b.iter(|| string_split_whitespace(black_box(&test_string)))
    });
    
    group.bench_function("string_split_char", |b| {
        b.iter(|| string_split_char(black_box(&test_string)))
    });
    
    group.bench_function("string_replace_naive", |b| {
        b.iter(|| string_replace_naive(black_box(&test_string), black_box("word"), black_box("term")))
    });
    
    group.bench_function("string_replace_builtin", |b| {
        b.iter(|| string_replace_builtin(black_box(&test_string), black_box("word"), black_box("term")))
    });
    
    group.bench_function("string_uppercase_naive", |b| {
        b.iter(|| string_uppercase_naive(black_box(&large_string)))
    });
    
    group.bench_function("string_uppercase_builtin", |b| {
        b.iter(|| string_uppercase_builtin(black_box(&large_string)))
    });
    
    group.bench_function("string_lowercase_naive", |b| {
        b.iter(|| string_lowercase_naive(black_box(&large_string)))
    });
    
    group.bench_function("string_lowercase_builtin", |b| {
        b.iter(|| string_lowercase_builtin(black_box(&large_string)))
    });
    
    group.bench_function("string_trim_naive", |b| {
        b.iter(|| string_trim_naive(black_box(&test_string)))
    });
    
    group.bench_function("string_trim_builtin", |b| {
        b.iter(|| string_trim_builtin(black_box(&test_string)))
    });
    
    group.bench_function("string_contains_naive", |b| {
        b.iter(|| string_contains_naive(black_box(&large_string), black_box("test")))
    });
    
    group.bench_function("string_contains_builtin", |b| {
        b.iter(|| string_contains_builtin(black_box(&large_string), black_box("test")))
    });
    
    group.finish();
}

criterion_group!(
    benches,
    sorting_benchmarks,
    parsing_benchmarks,
    algorithms_benchmarks,
    data_structures_benchmarks,
    string_operations_benchmarks
);
criterion_main!(benches); 