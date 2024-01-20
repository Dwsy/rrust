use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

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

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
