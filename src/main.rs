use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::search;

struct Config{
    query : String,
    file_path : String
}

fn main(){
    let args:Vec<String> = env::args().collect();
    
    let config : Config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing document {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}


impl Config{
    fn build(args : &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not enough arguments")
        }
        Ok(Config { query: args[1].clone(), file_path: args[2].clone() })
    }
}

// fn parse_config(args : &[String]) -> Config{
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path}
// }

fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents){
        println!("{line}")
    }

    Ok(())
}
