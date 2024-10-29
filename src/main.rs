use clap::Arg;
use clap::ArgAction;
use clap::Command;
use rlux::interpreter::Interpreter;
use core::str;
use std::fs;
use std::io::{self};
use std::path::Path;
use rustyline::error::ReadlineError;

fn main() {
    let matches = Command::new("rlux")
        .version("1.0")
        .author("Author Name <frankhampusweslien@gmail.com>")
        .about("Does awesome things")
        .subcommand(
            Command::new("run").about("Runs the application").arg(
                Arg::new("filepath")
                    .help("The path to the file to run")
                    .action(ArgAction::Set)
                    .value_name("FILE")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(Command::new("repl").about("Starts a REPL"))
        .get_matches();

    match matches.subcommand() {
        Some(("run", args)) => match args.get_one::<String>("filepath") {
            Some(filepath) => {
                println!("Running with file: {}", filepath);
                run_file(filepath).expect("Error running file");
            }
            None => println!("No filepath was provided"),
        },
        Some(("repl", _)) => {
            println!("Starting REPL...");
            run_prompt();
        }
        _ => println!("No valid subcommand was used"),
    }
}

fn run_prompt() {
    let mut interpreter = Interpreter::new();
    let mut rl = rustyline::DefaultEditor::new().expect("Failed to create editor");

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match rlux::run(line.trim(), &mut interpreter) {
                    Some(v) => println!("{}", v.to_string()),
                    None => (),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read(Path::new(path))?;
    let content = str::from_utf8(&bytes).expect("Invalid UTF-8 sequence");
    let mut interpreter = Interpreter::new();
    rlux::run(content, &mut interpreter);
    Ok(())
}
