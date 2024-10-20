use clap::Arg;
use clap::ArgAction;
use clap::Command;
use rlux::environment::Environment;
use core::str;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

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
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut env = Environment::new();
    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                match rlux::run(line.trim(), &mut env) {
                    Some(v) => println!("{}", v.to_string()),
                    None => (),
                }
            },
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                break;
            }
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read(Path::new(path))?;
    let content = str::from_utf8(&bytes).expect("Invalid UTF-8 sequence");
    let mut env = Environment::new();
    rlux::run(content, &mut env);
    Ok(())
}
