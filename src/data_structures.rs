use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, BinaryHeap};

pub fn hash_map_operations(size: usize) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..size {
        map.insert(i as i32, format!("value_{}", i));
    }
    map
}

pub fn btree_map_operations(size: usize) -> BTreeMap<i32, String> {
    let mut map = BTreeMap::new();
    for i in 0..size {
        map.insert(i as i32, format!("value_{}", i));
    }
    map
}

pub fn hash_set_operations(size: usize) -> HashSet<i32> {
    let mut set = HashSet::new();
    for i in 0..size {
        set.insert(i as i32);
    }
    set
}

pub fn btree_set_operations(size: usize) -> BTreeSet<i32> {
    let mut set = BTreeSet::new();
    for i in 0..size {
        set.insert(i as i32);
    }
    set
}

pub fn vec_operations(size: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(size);
    for i in 0..size {
        vec.push(i as i32);
    }
    vec
}

pub fn vec_deque_operations(size: usize) -> VecDeque<i32> {
    let mut deque = VecDeque::with_capacity(size);
    for i in 0..size {
        deque.push_back(i as i32);
    }
    deque
}

pub fn binary_heap_operations(size: usize) -> BinaryHeap<i32> {
    let mut heap = BinaryHeap::with_capacity(size);
    for i in 0..size {
        heap.push(i as i32);
    }
    heap
}

pub fn linked_list_operations(size: usize) -> Vec<i32> {
    let mut list = Vec::new();
    for i in 0..size {
        list.push(i as i32);
    }
    list
}

pub fn stack_operations(size: usize) -> Vec<i32> {
    let mut stack = Vec::new();
    for i in 0..size {
        stack.push(i as i32);
    }
    stack
}

pub fn queue_operations(size: usize) -> VecDeque<i32> {
    let mut queue = VecDeque::new();
    for i in 0..size {
        queue.push_back(i as i32);
    }
    queue
}

pub fn hash_map_lookup(map: &HashMap<i32, String>, keys: &[i32]) -> usize {
    let mut found = 0;
    for &key in keys {
        if map.contains_key(&key) {
            found += 1;
        }
    }
    found
}

pub fn btree_map_lookup(map: &BTreeMap<i32, String>, keys: &[i32]) -> usize {
    let mut found = 0;
    for &key in keys {
        if map.contains_key(&key) {
            found += 1;
        }
    }
    found
}

pub fn vec_search(vec: &[i32], targets: &[i32]) -> usize {
    let mut found = 0;
    for &target in targets {
        if vec.contains(&target) {
            found += 1;
        }
    }
    found
}

pub fn hash_set_contains(set: &HashSet<i32>, targets: &[i32]) -> usize {
    let mut found = 0;
    for &target in targets {
        if set.contains(&target) {
            found += 1;
        }
    }
    found
}

pub fn generate_search_keys(size: usize) -> Vec<i32> {
    let mut keys = Vec::with_capacity(size);
    for i in 0..size {
        keys.push((i * 2) as i32);
    }
    keys
} 