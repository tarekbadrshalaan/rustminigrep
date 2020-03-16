use rustminigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(e) = rustminigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
