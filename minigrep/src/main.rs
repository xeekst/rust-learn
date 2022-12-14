use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }

// fn run(config: Config) -> Result<(),Box<dyn Error>>{
//     let contents = fs::read_to_string(config.filename)?;

//     println!("With text:\n{}", contents);
//     Ok(())
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }

// fn parse_config_tuple(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename)
// }
