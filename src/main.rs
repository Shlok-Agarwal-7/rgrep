use std::{env, error::Error, fs, path::PathBuf, process};
use clap::{Arg, ArgAction, ArgMatches, command};
use minigrep::{search,search_insensitive,SearchResult};


pub enum IgnoreCase{
    SearchInsensitive,
    SearchSensitive
}

pub struct SearchConfig{
    filepaths : Vec<PathBuf>,
    query : String,
    ignore_case : IgnoreCase,
    line_number : bool,
    invert_match : bool
}

impl SearchConfig {
    pub fn build(matches : ArgMatches) -> SearchConfig{
        let  query = matches.get_one::<String>("query").expect("query cannot be empty");
        let  filepaths  = matches.get_many::<PathBuf>("filepath").unwrap_or_default().cloned().collect();
        let ignore_case = if matches.get_flag("ignore-case") {
                    IgnoreCase::SearchInsensitive
                } else {
                    IgnoreCase::SearchSensitive
                };
        let line_number = matches.get_flag("line-number");
        let invert_match = matches.get_flag("invert_match");
        SearchConfig { filepaths, 
                       query: (query.clone()), 
                       ignore_case,
                       line_number,
                       invert_match}
    }
}
fn main() {
    let query = Arg::new("query").required(true);

    let filepath = Arg::new("filepath")
                                .required(true)
                                .value_parser(clap::value_parser!(PathBuf))
                                .action(ArgAction::Set)
                                .num_args(1..);

    let ignore_case = Arg::new("ignore-case").short('i').long("ignore-case").action(ArgAction::SetTrue);

    let enable_line_number = Arg::new("line-number").short('n').long("line-number").action(ArgAction::SetTrue);

    let invert_match = Arg::new("invert_match").short('v').long("invert").action(ArgAction::SetTrue);

    let matches = command!().about("This program can be used to search your file systems").
                            arg(query).
                            arg(filepath).
                            arg(ignore_case).
                            arg(enable_line_number).
                            arg(invert_match).
                            get_matches();
    
    let config = SearchConfig::build(matches);

    if let Err(e) = run(config){
        eprint!("there was a problem reading the file :{e}");
        process::exit(1)
    }
}


fn run (config : SearchConfig) -> Result<(),Box<dyn Error>>{
   for filepath in &config.filepaths{
        let filename = filepath
            .file_name()
            .ok_or("path does not contain a filename")?
            .to_str()
            .ok_or("path is not valid UTF-8")?;

        let contents = fs::read_to_string(filepath)?;
        
        let result = match  config.ignore_case{
                IgnoreCase::SearchInsensitive => search_insensitive(&config.query, &contents,config.invert_match),
                IgnoreCase::SearchSensitive => search(&config.query, &contents,config.invert_match)
        };

        for SearchResult{line_number,line} in result{
                if config.line_number{
                    println!("{}::{}:{} ",filename,line_number,line);
                }
                else{
                    println!("{}::{}",filename,line);
                }
        }
   }

   Ok(())
}

