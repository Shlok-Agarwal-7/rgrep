use std::{env, error::Error, fs, process};
use clap::{Arg, ArgAction, ArgMatches, command};
use minigrep::{search,search_insensitive};


pub enum IgnoreCase{
    SearchInsensitive,
    SearchSensitive
}

pub struct SearchConfig{
    filepath : String,
    query : String,
    ignorecase : IgnoreCase,
    linenumber : bool
}


impl SearchConfig {
    pub fn build(matches : ArgMatches) -> SearchConfig{
        let  query = matches.get_one::<String>("query").expect("query cannot be empty");
        let  filepath=matches.get_one::<String>("filepath").expect("file path cannot be empty");
        let ignorecase = if matches.get_flag("ignore-case") {
                    IgnoreCase::SearchInsensitive
                } else {
                    IgnoreCase::SearchSensitive
                };

        let linenumber = matches.get_flag("line-number");
        SearchConfig { filepath: (filepath.clone()), 
                       query: (query.clone()), 
                       ignorecase,
                       linenumber}
    }
}
fn main() {
    let query = Arg::new("query").required(true);

    let filepath = Arg::new("filepath").required(true);

    let ignorecase = Arg::new("ignore-case").short('i').long("ignore-case").action(ArgAction::SetTrue);

    let enable_line_number = Arg::new("line-number").short('n').long("line-number").action(ArgAction::SetTrue);

    let matches = command!().about("This program can be used to search your file systems").
                            arg(query).
                            arg(filepath).
                            arg(ignorecase).
                            arg(enable_line_number).
                            get_matches();
    
    let config = SearchConfig::build(matches);

    if let Err(e) = run(config){
        eprint!("there was a problem reading the file :{e}");
        process::exit(1)
    }
}


fn run (config : SearchConfig) -> Result<(),Box<dyn Error>>{
   let contents = fs::read_to_string(config.filepath)?;
   
   let result = match  config.ignorecase{
        IgnoreCase::SearchInsensitive => search_insensitive(&config.query, &contents),
        IgnoreCase::SearchSensitive => search(&config.query, &contents)
   };

   for line in result{
        if config.linenumber{
            println!("{}",line);
        }
        else{
            println!("{}",line);
        }
   }

   Ok(())
}

