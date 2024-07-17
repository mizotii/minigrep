use std::error::Error;
use std::fs;

// stores args
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> { // error values are always string
                                                                // literals with a static litetime
        // ensure proper usage
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    Ok(())
}

// data returned by the search function will live as long as the data from contents
// without lifetime annotations the compiler could not figure out which parameter
// the return value was borrowed from
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterate through each line of contents
    
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // we need contents to live as long as the product of search
        assert_eq!(vec!["safe", "fast", "productive."], search(query, contents));
    }
}