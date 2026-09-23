use std::{env, error::Error, fs, process};
use clap::{Arg, ArgAction, ArgMatches, command};
use minigrep::{search,search_insensitive};

fn main() {
    let query = Arg::new("query").required(true);

    let filepath = Arg::new("filepath").required(true);

    let ignorecase = Arg::new("ignore-case").short('i').long("ignore_case").action(ArgAction::SetTrue);

    let matches = command!().about("This program can be used to search your file systems").
                            arg(query).
                            arg(filepath).
                            arg(ignorecase).
                            get_matches();

    if let Err(e) = run(matches){
        eprint!("there was a problem reading the file :{e}");
        process::exit(1)
    }
}


fn run (matches : ArgMatches) -> Result<(),Box<dyn Error>>{


    let file_path = matches.get_one::<String>("filepath").expect("filepath is required");

    let contents = fs::read_to_string(file_path)?;

    let query = matches.get_one::<String>("query").expect("query is required");

    let ignore_case = matches.get_flag("ignore-case");

    if ignore_case {
        for line in search_insensitive( query, &contents){
            println!("{line}")
        }
    } 

    else{
        for line in search( query, &contents){
            println!("{line}")
        }
    }
   
   Ok(())

}

