mod pattern;
use pattern::ReAutomata;

pub fn find_pattern(input_line: String, pattern: &str) -> bool {
    let mut chars = pattern.chars();
    let mut automata = ReAutomata::from(&mut chars);
    !automata
        .all_matches(&input_line.chars().collect::<Vec<char>>())
        .is_empty()
}
