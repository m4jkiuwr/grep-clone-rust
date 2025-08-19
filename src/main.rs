use codecrafters_grep::find_pattern;
use std::env;
use std::io;
use std::process;

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if find_pattern(input_line, &pattern) {
        println!("Found");
        process::exit(0)
    } else {
        process::exit(1)
    }
}
