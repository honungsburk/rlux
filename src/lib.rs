pub mod expr;
pub mod expr_eval;
pub mod expr_parser;
pub mod parser;
pub mod position;
pub mod scanner;
pub mod token;

use scanner::Scanner;

pub fn run(source: &str) {
    let line_offsets = position::LineOffsets::new(source);

    let mut scanner = Scanner::new(source);

    let tokens = scanner.run();

    let expr = expr_parser::run(&tokens);

    match expr {
        Ok(expr) => {
            let value = expr_eval::run(&expr);
            match value {
                Ok(value) => println!("{:?}", value),
                Err(err) => eprintln!("{:?}", err),
            }
        }
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                eprintln!(
                    "Error: {} at {}",
                    diagnostic.message,
                    line_offsets.line(diagnostic.span.start)
                );
            }
        }
    }
}

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

pub fn report(line: u32, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}
