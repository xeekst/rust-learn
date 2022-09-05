use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
        if args.len() < 3 {
            eprintln!("errrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr");
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let query = "a";
/// let contents = "agafasf";
/// let answer = my_crate::search(query,contents);
///
/// assert_eq!("agafasf", answer);
/// ```
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|c| c.contains(query)).collect()
}
