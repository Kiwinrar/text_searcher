use std::env;
use std::process;
use text_searcher;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let parameters=match text_searcher::ConfigQuery::new(&arguments){
        Ok(queries)=>{
            queries
        },
        Err(e)=>{println!("Error parsing arguments: {}", e);process::exit(1)}
    };
    let (queries, filename)=parameters;
    let query_output=match text_searcher::ConfigQuery::parameters(queries, filename){
        Ok(query)=>{
            let (config_query, query_parameter)=query;
            (config_query, query_parameter)
        },
        Err(e)=>{
            println!("Error satifying the query: {}", e);
            process::exit(2);
        }
    };
    let (config_query, query_parameter)=query_output;
    if let Err(e) = text_searcher::run(config_query, query_parameter) {
        println!("Application Error: {}", e);
        process::exit(2)
    }
}
