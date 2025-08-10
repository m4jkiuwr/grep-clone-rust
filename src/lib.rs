mod pattern;
use pattern::Pattern;

pub fn find_pattern(input_line: &str, pattern: &str) -> bool {
    let pattern = Pattern::from(pattern);
    pattern.inside(input_line)
}
