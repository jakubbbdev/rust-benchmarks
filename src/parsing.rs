pub fn parse_integers_simple(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

pub fn parse_integers_with_validation(input: &str) -> Result<Vec<i32>, String> {
    let mut result = Vec::new();
    
    for (line_num, line) in input.lines().enumerate() {
        for (col_num, word) in line.split_whitespace().enumerate() {
            match word.parse::<i32>() {
                Ok(num) => result.push(num),
                Err(_) => return Err(format!("Invalid integer at line {}, column {}", line_num + 1, col_num + 1)),
            }
        }
    }
    
    Ok(result)
}

pub fn parse_csv_simple(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().to_string())
                .collect()
        })
        .collect()
}

pub fn parse_csv_with_quotes(input: &str) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current_row = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    current_field.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                current_row.push(current_field.trim().to_string());
                current_field.clear();
            }
            '\n' if !in_quotes => {
                current_row.push(current_field.trim().to_string());
                result.push(current_row);
                current_row = Vec::new();
                current_field.clear();
            }
            _ => current_field.push(ch),
        }
    }
    
    if !current_field.is_empty() || !current_row.is_empty() {
        current_row.push(current_field.trim().to_string());
        result.push(current_row);
    }
    
    result
}

pub fn generate_test_data(size: usize) -> String {
    let mut result = String::new();
    for i in 0..size {
        result.push_str(&format!("{}, {}, {}\n", i, i * 2, i * 3));
    }
    result
}

pub fn generate_csv_data(rows: usize, cols: usize) -> String {
    let mut result = String::new();
    for row in 0..rows {
        for col in 0..cols {
            if col > 0 {
                result.push(',');
            }
            result.push_str(&format!("field_{}_{}", row, col));
        }
        result.push('\n');
    }
    result
} 