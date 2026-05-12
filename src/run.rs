use std::error::Error;
use std::fs;
use text_searcher;

pub fn run(
    config_parameter: text_searcher::ConfigQuery,
    search_parameter: &[String]
) -> Result<(), Box<dyn Error>> {
    let contents_of_file = fs::read_to_string(config_parameter.filename)?;
    if matches!(config_parameter.query, text_searcher::Commands::Search) {
        search_execution(&contents_of_file, search_parameter);
    }
    if matches!(config_parameter.query, text_searcher::Commands::Read) {
        read_execution(&contents_of_file);
    }
    Ok(())
}
pub fn search_execution(contents_of_file:&str, search_parameter: &[String]){
        let search_result = search(search_parameter, &contents_of_file);
        for val in search_result.iter() {
            println!("{}", val);
        }
}
pub fn read_execution(contents_of_file:&str){
    println!("{}", contents_of_file);
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