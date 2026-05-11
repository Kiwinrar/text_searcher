use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let parameters = match ConfigQuery::new(&arguments) {
        Ok(value) => {
            println!("The given query: {:?}", value.query);
            value
        }
        Err(e) => {
            println!("Error parsing the arguments: {}", e);
            process::exit(1)
        }
    };
    if let Err(e) = run(parameters) {
        println!("Application Error: {}", e);
        process::exit(2)
    }
}
fn run(config_parameter: ConfigQuery) -> Result<(), Box<dyn Error>> {
    let contents_of_file = fs::read_to_string(config_parameter.filename)?;
    println!("{}", contents_of_file);
    Ok(())
}
struct ConfigQuery<'a> {
    query: Commands,
    filename: &'a str,
}
#[derive(Debug)]
pub enum Commands{
    Read,
}
impl<'a> ConfigQuery<'a> {
    fn new(args: &'a Vec<String>) -> Result<ConfigQuery<'a>, &'a str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: &str = &args[1];
        let command_query=match query{
            "read"=>Commands::Read,
            &_=>{return Err("Invalid query");}
        };
        let filename: &str = &args[2];
        let parameter = ConfigQuery {
            query: command_query,
            filename,
        };
        Ok(parameter)
    }
}
