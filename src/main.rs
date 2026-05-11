use std::env;
use std::process;
use text_searcher;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let parameters = match text_searcher::ConfigQuery::new(&arguments) {
        Ok(value) => {
            let (val, search_parameter) = value;
            println!("The given query: {:?}", val.query);
            if search_parameter != "".to_string() {
                println!("Search parameter: {}", search_parameter);
            }
            (val,search_parameter)
        }
        Err(e) => {
            println!("Error parsing the arguments: {}", e);
            process::exit(1)
        }
    };
    let (val, search_parameter)=parameters;
    if let Err(e) = text_searcher::run(val, &search_parameter) {
        println!("Application Error: {}", e);
        process::exit(2)
    }
}
