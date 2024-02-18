use std::env;
use std::process;
use minigrep::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });    
    println!("Search for: {}", config.query);
    println!("Search in: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

