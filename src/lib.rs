pub mod expr;
pub mod expr_eval;
pub mod expr_parser;
pub mod parser;
pub mod position;
pub mod scanner;
pub mod token;
pub mod stmt;
pub mod stmt_parser;
pub mod stmt_eval;
pub mod program;
pub mod run_time_error;

use scanner::Scanner;

pub fn run(source: &str) {
    let line_offsets = position::LineOffsets::new(source);

    let mut scanner = Scanner::new(source);

    let tokens = scanner.run();

    let program = program::Program::parse(&tokens);

    match program {
        Ok(p) => {
            match p.run() {
                Ok(()) => {},
                Err(err) => eprintln!("{:?}", err),
            }
        }
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                eprintln!(
                    "Error: {} at line {}",
                    diagnostic.message,
                    line_offsets.line(diagnostic.span.start)
                );
            }
        }
    }
}