use std::error::Error;
use std::fs;

pub fn run(
    config_parameter: ConfigQuery,
    search_parameter: &[String]
) -> Result<(), Box<dyn Error>> {
    let contents_of_file = fs::read_to_string(config_parameter.filename)?;
    if matches!(config_parameter.query, Commands::Search) {
        let search_result = search(search_parameter, &contents_of_file);
        for val in search_result.iter() {
            println!("{}", val);
        }
    }
    if matches!(config_parameter.query, Commands::Read) {
        println!("{}", contents_of_file);
    }
    Ok(())
}
pub fn search<'a>(query: &'a [String], contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for (_i, ch) in query.iter().enumerate() {
        for line in contents.lines(){
            if line.contains(ch){
                results.push(line);
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
    pub fn new(args: &'a Vec<String>) -> Result<(&'a [String], &'a String), &'a str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let mut file_index = None;
        for (i, val) in args.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if val.contains(".txt") {
                file_index = Some(i);
                break;
            }
        }
        let index = match file_index {
            Some(i) => i,
            None => {
                return Err("File name missing");
            }
        };
        let query = &args[1..index];
        let filename = &args[index];

        Ok((query, filename))
    }

    pub fn parameters(
        queries: &'a [String],
        filename: &'a String
    ) -> Result<(ConfigQuery<'a>, &'a [String]), &'a str> {
        let command_query = &queries[0];
        
        let config_command = match command_query.as_str() {
            "read" => Commands::Read,
            "search" => Commands::Search,
            _ => {
                return Err("Invalid Command query");
            }
        };
        let query_parameter = &queries[1..queries.len()];
        
        if query_parameter.is_empty() {
            if matches!(config_command, Commands::Search) {
                return Err("The arguments to the query are missing");
            }
        }
        let config_parameter = ConfigQuery {
            query: config_command,
            filename,
        };
        Ok((config_parameter, query_parameter))
    }
}
