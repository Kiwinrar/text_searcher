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
            let (config_query, queries_array)=query;
            (config_query, queries_array)
        },
        Err(e)=>{
            println!("Error satifying the query: {}", e);
            process::exit(2);
        }
    };
    let (val, search_parameter)=query_output;
    if let Err(e) = text_searcher::run(val, search_parameter) {
        println!("Application Error: {}", e);
        process::exit(2)
    }
}
