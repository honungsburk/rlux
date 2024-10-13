pub mod expr;
pub mod expr_parser;
pub mod parser;
pub mod position;
pub mod scanner;
pub mod token;

use scanner::Scanner;

pub fn run(source: &str) {
    let mut scanner = Scanner::new(source);

    let tokens = scanner.run();

    // For now, just print the tokens.
    for token in tokens {
        println!("{}", token.value);
    }
}

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

pub fn report(line: u32, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}
