pub fn find_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern == r"\d" {
        input_line.chars().any(|c| c.is_numeric())
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}
