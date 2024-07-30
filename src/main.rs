use std::env;
use std::fs;
use std::process;
use std::error::Error;
struct Config {
    _query: String,
    filename: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args).unwrap_or_else(|err:&str|{
        println!("problem parsing the argument {}",err);
        process::exit(1);
    });

    if let Err(e)=run(config)
    {
        println!("Application error:{}",e);
        process::exit(1)

    }

    println!("{:?}", args);
}


fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    Ok(())


}



fn parse_config(args: &[String]) -> Result<Config, &str> {// parse the argument ,extract and return the query and filename
    if args.len() < 3 {
        return Err("Not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok( Config { query, filename })
}
