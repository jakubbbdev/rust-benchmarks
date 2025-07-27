pub fn string_concat_plus(size: usize) -> String {
    let mut result = String::new();
    for i in 0..size {
        result = result + &format!("word_{}", i);
    }
    result
}

pub fn string_concat_push_str(size: usize) -> String {
    let mut result = String::new();
    for i in 0..size {
        result.push_str(&format!("word_{}", i));
    }
    result
}

pub fn string_concat_format(size: usize) -> String {
    let mut result = String::new();
    for i in 0..size {
        result = format!("{}{}", result, format!("word_{}", i));
    }
    result
}

pub fn string_concat_collect(size: usize) -> String {
    let parts: Vec<String> = (0..size).map(|i| format!("word_{}", i)).collect();
    parts.join("")
}

pub fn string_reverse_naive(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn string_reverse_bytes(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut reversed = Vec::with_capacity(bytes.len());
    for &byte in bytes.iter().rev() {
        reversed.push(byte);
    }
    String::from_utf8(reversed).unwrap_or_default()
}

pub fn string_split_whitespace(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn string_split_char(s: &str) -> Vec<&str> {
    s.split(' ').collect()
}

pub fn string_replace_naive(s: &str, from: &str, to: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < s.len() {
        if s[i..].starts_with(from) {
            result.push_str(to);
            i += from.len();
        } else {
            result.push(s.chars().nth(i).unwrap_or(' '));
            i += 1;
        }
    }
    result
}

pub fn string_replace_builtin(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn string_uppercase_naive(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        result.push(ch.to_ascii_uppercase());
    }
    result
}

pub fn string_uppercase_builtin(s: &str) -> String {
    s.to_uppercase()
}

pub fn string_lowercase_naive(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        result.push(ch.to_ascii_lowercase());
    }
    result
}

pub fn string_lowercase_builtin(s: &str) -> String {
    s.to_lowercase()
}

pub fn string_trim_naive(s: &str) -> String {
    let mut start = 0;
    let mut end = s.len();
    
    while start < end && s.chars().nth(start).map_or(false, |c| c.is_whitespace()) {
        start += 1;
    }
    
    while end > start && s.chars().nth(end - 1).map_or(false, |c| c.is_whitespace()) {
        end -= 1;
    }
    
    s[start..end].to_string()
}

pub fn string_trim_builtin(s: &str) -> String {
    s.trim().to_string()
}

pub fn generate_test_string(size: usize) -> String {
    let mut result = String::new();
    for i in 0..size {
        result.push_str(&format!("word_{} ", i));
    }
    result
}

pub fn generate_large_string(size: usize) -> String {
    let mut result = String::new();
    for i in 0..size {
        result.push_str(&format!("This is a longer test string with number {} and some additional text to make it more realistic. ", i));
    }
    result
}

pub fn string_contains_naive(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    
    for i in 0..=haystack.len().saturating_sub(needle.len()) {
        if haystack[i..].starts_with(needle) {
            return true;
        }
    }
    false
}

pub fn string_contains_builtin(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
} 