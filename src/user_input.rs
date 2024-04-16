use std::io::Stdin;
use std::io::stdin;

///
/// Simple function just to grab user input and convert it to a standard format used throughout the code
///
pub fn input() -> String {
    let mut input: String = String::new();
    let stdin: Stdin = stdin();
    stdin.read_line(&mut input).unwrap();
    input.trim().to_ascii_lowercase()
}
