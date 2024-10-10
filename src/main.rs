use clap::Arg;
use clap::ArgAction;
use clap::Command;

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
        Some(("run", args)) => {
            match args.get_one::<String>("filepath") {
                Some(filepath) => {
                    println!("Running with file: {}", filepath);
                    // Add your file processing logic here
                }
                None => println!("No filepath was provided"),
            }
        }
        Some(("repl", _)) => {
            println!("Starting REPL...");
            // Add your REPL logic here
        }
        _ => println!("No valid subcommand was used"),
    }
}
