pub mod scanner;

pub fn run(source: &str) {
    let tokens = scanner::run(source);

    // For now, just print the tokens.
    for token in tokens {
        println!("{}", token);
    }
}
