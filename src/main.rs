use colored::*;
use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     panic!("Not enough arguments");
    // } else if args.len() > 3 {
    //     panic!("Not enough arguments");
    // }
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    contents.lines().for_each(|line| {
        if line.contains(config.query) {
            highlight_search(line, config.query);
        }
    });
}

fn highlight_search(text: &str, search: &str) {
    let highlighted = text.replace(search, &format!("{}", search.red()));
    println!("{}", highlighted);
}
#[derive()]
struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a Vec<String>) -> Self {
        let query = if args.len() > 1 { &args[1] } else { "漆黑" };
        let file_path = if args.len() > 2 {
            &args[2]
        } else {
            "Explosion.txt"
        };
        Self { query, file_path }
    }
}
