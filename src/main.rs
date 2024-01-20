use minigrep::*;
use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     panic!("Not enough arguments");
    // } else if args.len() > 3 {
    //     panic!("Not enough arguments");
    // }
    let config = minigrep::Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    search_text(&contents, &config);
}
