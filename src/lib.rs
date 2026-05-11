use std::error::Error;
use std::fs;

pub fn run(config_parameter: ConfigQuery, search_parameter: &str) -> Result<(), Box<dyn Error>> {
    let contents_of_file = fs::read_to_string(config_parameter.filename)?;
    if matches!(config_parameter.query, Commands::Search) {
        let search_result = search(search_parameter, &contents_of_file);
        for val in search_result.iter(){
            println!("{}", val);
        }
    }
    if matches!(config_parameter.query, Commands::Read) {
        println!("{}", contents_of_file);
    }
    Ok(())
}
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for (i, ch) in query.chars().enumerate() {
        if ch == ' ' {
            let str = &query[i+1..query.len()];
            for line in contents.lines() {
                if line.contains(str) {
                    results.push(line);
                }
            }
        }
    }
    results
}
pub struct ConfigQuery<'a> {
    pub query: Commands,
    filename: &'a str,
}
#[derive(Debug)]
pub enum Commands {
    Read,
    Search,
}
impl<'a> ConfigQuery<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<(ConfigQuery<'a>, String), &'a str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: &str = &args[1];
        let command_query = match query {
            "read" => Commands::Read,
            "search" => Commands::Search,
            &_ => {
                return Err("Invalid query");
            }
        };
        let word;
        let filename: &String;
        let search_query;
        match command_query {
            Commands::Read => {
                filename = &args[2];
                search_query = "".to_string();
            }
            Commands::Search => {
                if args.len() < 4 {
                    return Err("The search argument is not provided");
                }
                word = &args[2];
                search_query = format!("{} {}", query, word);
                filename = &args[3];
            }
        }
        let parameter = ConfigQuery {
            query: command_query,
            filename: filename,
        };
        Ok((parameter, search_query))
    }
}
