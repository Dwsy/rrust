use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
#[derive()]
struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    fn new(query: &'a str, file_path: &'a str) -> Self {
        Self { query, file_path }
    }
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = if args.len() > 1 {
        &args[1]
    } else {
        "Explosion"
    };
    let file_path = if args.len() > 2 {
        &args[2]
    } else {
        "Explosion.txt"
    };
    Config::new(query, file_path)
}
