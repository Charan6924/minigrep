use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::{search, search_case_insensitive};

pub struct Config{
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool
}

fn main(){
    let args:Vec<String> = env::args().collect();
    
    let config : Config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing document {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}


impl Config{
    fn build(args : &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not enough arguments")
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query: args[1].clone(), file_path: args[2].clone(), ignore_case: ignore_case})
    }
}

// fn parse_config(args : &[String]) -> Config{
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path}
// }

fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case{
        for line in search_case_insensitive(&config.query, &contents){
            println!("{line}")
        }
    }else{
        for line in search(&config.query, &contents){
            println!("{line}")
        }
    }

    Ok(())
}
