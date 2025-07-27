use rust_benchmarks::{
    bubble_sort, generate_csv_data, generate_random_array, generate_test_data, merge_sort,
    parse_csv_simple, parse_csv_with_quotes, parse_integers_simple, parse_integers_with_validation,
    quick_sort,
};
use std::time::Instant;

fn main() {
    println!("Running manual benchmarks...\n");
    
    run_sorting_benchmarks();
    println!();
    run_parsing_benchmarks();
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