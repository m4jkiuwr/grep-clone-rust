mod pattern;
use pattern::ReAutomata;

pub fn find_pattern(input_line: String, pattern: &str) -> bool {
    let mut chars = pattern.chars();
    let input: Vec<char> = input_line.chars().collect();
    let mut automata = ReAutomata::from(&mut chars);
    // println!("{:#?}", automata);
    let matches = automata.all_matches(&input);

    let res = !matches.is_empty();

    for (a, b) in matches {
        println!("{} {}", a, b);
    }

    res // Return the actual result
}
