pub mod expr;
pub mod expr_parser;
pub mod parser;
pub mod position;
pub mod scanner;
pub mod token;
pub mod stmt;
pub mod stmt_parser;
pub mod program;
pub mod interpreter;
use scanner::Scanner;
use interpreter::{Interpreter, LuxValue};

pub fn run(source: &str, interpreter: &mut Interpreter) -> Option<LuxValue> {
    let line_offsets = position::LineOffsets::new(source);

    let mut scanner = Scanner::new(source);

    let tokens = scanner.run();

    let program = program::Program::parse(&tokens);

    match program {
        Ok(p) => {
            match interpreter.run(p) {
                Ok(v) => v,
                Err(err) => { 
                    eprintln!("{:?}", err); 
                    None 
                }
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
            None
        }
    }
}