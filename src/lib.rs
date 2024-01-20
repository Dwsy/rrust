use colored::Colorize;

pub fn search_text(contents: &str, config: &Config) -> () {
    contents.lines().for_each(|line| {
        if line.contains(config.query) {
            highlight_search(line, config.query);
        }
    });
}

pub fn highlight_search(text: &str, search: &str) {
    let highlighted = text.replace(search, &format!("{}", search.red()));
    println!("{}", highlighted);
}
#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Self {
        let query = if args.len() > 1 { &args[1] } else { "漆黑" };
        let file_path = if args.len() > 2 {
            &args[2]
        } else {
            "Explosion.txt"
        };
        Self { query, file_path }
    }
}
