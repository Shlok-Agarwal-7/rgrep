use std::{env, error::Error, fs, path::PathBuf, process};
use clap::{Arg, ArgAction,command};
use minigrep::search::{search,search_insensitive,SearchResult};
use minigrep::config::{CaseSensitivity,SearchConfig};

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
        
        let result = match  config.case_sensitivity{
                CaseSensitivity::Insensitive => search_insensitive(&config.query, &contents,config.invert_match),
                CaseSensitivity::Sensitive => search(&config.query, &contents,config.invert_match)
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

