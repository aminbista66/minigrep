use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });    

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("Search for: {}", config.query);
    println!("Search in:\n{contents}");
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough parameters");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Self {query, file_path})
    }
}
