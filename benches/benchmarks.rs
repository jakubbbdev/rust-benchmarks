use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_benchmarks::{
    bubble_sort, generate_csv_data, generate_random_array, generate_test_data, merge_sort,
    parse_csv_simple, parse_csv_with_quotes, parse_integers_simple, parse_integers_with_validation,
    quick_sort,
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

criterion_group!(benches, sorting_benchmarks, parsing_benchmarks);
criterion_main!(benches); 