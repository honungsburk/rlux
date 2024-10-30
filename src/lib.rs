pub mod expr_parser;
pub mod parser;
pub mod position;
pub mod scanner;
pub mod token;
pub mod ast;
pub mod stmt_parser;
pub mod program;
pub mod interpreter;
pub mod resolver;

use position::{Diagnostic, Span};
use resolver::Resolver;
use scanner::Scanner;
use interpreter::{Interpreter, LuxValue};

pub fn run(source: &str, interpreter: &mut Interpreter) -> Option<LuxValue> {
    let line_offsets = position::LineOffsets::new(source);

    let mut scanner = Scanner::new(source);

    let tokens = scanner.run();

    let result = program::Program::parse(&tokens).and_then(|p| {
            Resolver::new(interpreter).run(&p)?;
            Ok(p)
        }).and_then(|p| {
            interpreter.run(&p)
                .map_err(|err| {
                    vec![
                        Diagnostic {
                            span: Span::empty(), 
                            message: format!("{:?}", err)
                        }
                        ]
                    }
                )
        });
    
    match result {
        Ok(v) => {
            v
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