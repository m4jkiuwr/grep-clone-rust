pub fn find_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else {
        match pattern {
            r"\d" => input_line.chars().any(|c| c.is_numeric()),
            r"\w" => input_line.chars().any(|c| c.is_alphanumeric()),
            _ => todo!(),
        }
    }
}
