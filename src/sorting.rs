pub fn bubble_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let mut result = arr.to_vec();
    let len = result.len();
    
    for i in 0..len {
        for j in 0..len - i - 1 {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1);
            }
        }
    }
    result
}

pub fn quick_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    
    let pivot = &arr[arr.len() / 2];
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut equal = Vec::new();
    
    for item in arr {
        match item.cmp(pivot) {
            std::cmp::Ordering::Less => left.push(item.clone()),
            std::cmp::Ordering::Equal => equal.push(item.clone()),
            std::cmp::Ordering::Greater => right.push(item.clone()),
        }
    }
    
    let mut result = quick_sort(&left);
    result.extend(equal);
    result.extend(quick_sort(&right));
    result
}

pub fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    
    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);
    
    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

pub fn generate_random_array(size: usize) -> Vec<i32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut result = Vec::with_capacity(size);
    for i in 0..size {
        let mut hasher = DefaultHasher::new();
        i.hash(&mut hasher);
        result.push((hasher.finish() % 10000) as i32);
    }
    result
} 