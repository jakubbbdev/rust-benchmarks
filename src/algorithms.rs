pub fn fibonacci_recursive(n: u64) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

pub fn fibonacci_iterative(n: u64) -> u128 {
    if n <= 1 {
        return n as u128;
    }
    
    let mut a = 0u128;
    let mut b = 1u128;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

pub fn fibonacci_dynamic(n: u64) -> u128 {
    if n <= 1 {
        return n as u128;
    }
    
    let mut memo = vec![0u128; (n + 1) as usize];
    memo[1] = 1;
    
    for i in 2..=n {
        memo[i as usize] = memo[(i - 1) as usize] + memo[(i - 2) as usize];
    }
    
    memo[n as usize]
}

pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    arr.iter().position(|x| x == target)
}

pub fn prime_sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    let sqrt_n = (n as f64).sqrt() as usize;
    for i in 2..=sqrt_n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

pub fn gcd_euclidean(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn gcd_naive(a: u64, b: u64) -> u64 {
    let min_val = a.min(b);
    for i in (1..=min_val).rev() {
        if a % i == 0 && b % i == 0 {
            return i;
        }
    }
    1
}

pub fn matrix_multiply_naive(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();
    
    let mut result = vec![vec![0; cols_b]; rows_a];
    
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

pub fn generate_matrix(size: usize) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = ((i * j) % 100) as i32;
        }
    }
    matrix
} 