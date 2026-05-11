use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let parameter = ConfigQuery::new(&arguments);
    match parameter {
        Ok(value) => {
            value.query;
            if let Err(e)=run(value){
                println!("Application Error: {}", e);
                process::exit(2)
            }
        }
        Err(err) => {
            println!("Error parsing the parameters: {}", err);
            process::exit(1)
        }
    }
}
fn run(config_parameter: ConfigQuery)->Result<(), Box<dyn Error>> {
    let contents_of_file = fs::read_to_string(config_parameter.filename)?;
    println!("{}", contents_of_file);
    Ok(())
}
struct ConfigQuery<'a> {
    query: &'a str,
    filename: &'a str,
}
impl<'a> ConfigQuery<'a> {
    fn new(args: &'a Vec<String>) -> Result<ConfigQuery<'a>, &'a str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: &str = &args[1];
        let filename: &str = &args[2];
        let parameter = ConfigQuery {
            query,
            filename,
        };
        Ok(parameter)
    }
}
