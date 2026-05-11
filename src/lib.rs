use std::error::Error;
use std::fs;

pub fn run(config_parameter: ConfigQuery) -> Result<(), Box<dyn Error>> {
    let contents_of_file = fs::read_to_string(config_parameter.filename)?;
    println!("{}", contents_of_file);
    Ok(())
}
pub struct ConfigQuery<'a> {
    pub query: Commands,
    filename: &'a str,
}
#[derive(Debug)]
pub enum Commands{
    Read,
}
impl<'a> ConfigQuery<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<ConfigQuery<'a>, &'a str> {
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