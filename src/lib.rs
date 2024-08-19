use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn parse_config(args: &[String]) -> Result<Config, &str> {

    if args.len()<2 {
        return Err("Not enough arguments");
    }
    let query = args[0].clone();
    let filename = args[1].clone();

    Ok(Config { query, filename })
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_config_success() {
        let args = vec![
            String::from("program_name"),
            String::from("test_query"),
            String::from("test_file.txt"),
        ];
        let config = parse_config(&args).unwrap();
        assert_eq!(config.query, "test_query");
        assert_eq!(config.filename, "test_file.txt");
    }
    #[test]
    fn test_parse_config_not_enough_args() {
        let args = vec![String::from("program_name")];
        let result = parse_config(&args);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Not enough arguments");
    }

    #[test]
    fn test_run_file_not_found() {
        let config = Config {
            query: String::from("test_query"),
            filename: String::from("non_existent_file.txt"),
        };
        let result = run(config);
        assert!(result.is_err());
    }
    #[test]
    fn test_run_file_success() {
        let test_filename = "test_file.txt";
        let test_content = "Hello, world!";
        fs::write(test_filename, test_content).unwrap();

        let config = Config {
            query: String::from("test_query"),
            filename: String::from(test_filename),
        };
        let result = run(config);
        assert!(result.is_ok());

        fs::remove_file(test_filename).unwrap();
    }

    #[test]
    fn test_search_multiple_matches() {
        let query = "the";
        let contents = "\
there is a line.
ima tbnhe their.";

        let expected = vec!["there is a line.", "ima tbnhe their."];
        let result = search(query, contents);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_search_case_insensitive() {
        let query = "THe";
        let contents = "\
there is a line.
ima tbnhe their.";

        let expected = vec!["there is a line.", "ima tbnhe their."];
        let result = search(query, contents);
        assert_eq!(result, expected);
    }
}
