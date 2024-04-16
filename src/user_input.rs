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


///
/// Simple function to get a usize from the user
///
pub fn usize_input() -> usize {
    let mut input: String = String::new();
    let stdin: Stdin = stdin();
    stdin.read_line(&mut input).unwrap();
    match input.trim().parse::<usize>() {
        Ok(i) => i,
        Err(..) => { println!("\nPlease enter a whole number:"); usize_input()},
    }
}

///
/// Simple function to get an i32 from the user
///
pub fn int_input() -> i32 {
    let mut input: String = String::new();
    let stdin: Stdin = stdin();
    stdin.read_line(&mut input).unwrap();
    match input.trim().parse::<i32>() {
        Ok(i) => i,
        Err(..) => { println!("\nPlease enter a valid integer:"); int_input()},
    }
}
