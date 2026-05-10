use std::env;
use std::fs;
use std::process;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let parameter = ConfigQuery::new(&arguments);
    match parameter {
        Ok(value) => {
            value.query;
            let contents_of_file = fs::read_to_string(value.filename);
            match contents_of_file {
                Ok(contents) => println!("{}", contents),
                Err(er) => {
                    println!("Error reading the contents of the file:\n{}", er);
                    process::exit(2)
                }
            }
        }
        Err(err) => {
            println!("Error parsing the parameters: {}", err);
            process::exit(1)
        }
    }
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
        print!("{}", filename);
        let parameter = ConfigQuery {
            query,
            filename,
        };
        Ok(parameter)
    }
}
