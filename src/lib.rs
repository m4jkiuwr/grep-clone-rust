mod pattern;
use pattern::ReAutomata;

pub fn find_pattern(input_line: String, pattern: &str) -> bool {
    let mut automata = ReAutomata::from(pattern);
    automata.run(&input_line.chars().collect::<Vec<char>>())
}
