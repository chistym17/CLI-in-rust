use std::process;
use CLI::{run, parse_config};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("CLI Tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A simple CLI tool")
        .arg(Arg::new("query")
            .help("The query string to search for")
            .required(true)
            .index(1))
        .arg(Arg::new("filename")
            .help("The file to search in")
            .required(true)
            .index(2))
        .get_matches();


    let query = matches.get_one::<String>("query").unwrap();
    let filename = matches.get_one::<String>("filename").unwrap();

    let config = parse_config(&[query.to_string(), filename.to_string()]).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing the argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
