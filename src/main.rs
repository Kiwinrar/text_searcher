use std::env;
use std::process;
use text_searcher;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let parameters = match text_searcher::ConfigQuery::new(&arguments) {
        Ok(value) => {
            println!("The given query: {:?}", value.query);
            value
        }
        Err(e) => {
            println!("Error parsing the arguments: {}", e);
            process::exit(1)
        }
    };
if let Err(e) = text_searcher::run(parameters) {
        println!("Application Error: {}", e);
        process::exit(2)
    }
}