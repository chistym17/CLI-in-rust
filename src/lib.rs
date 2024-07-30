use std::fs;
use std::error::Error;
pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    Ok(())


}



pub fn parse_config(args: &[String]) -> Result<Config, &str> {// parse the argument ,extract and return the query and filename
    if args.len() < 3 {
        return Err("Not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok( Config { query, filename })
}
