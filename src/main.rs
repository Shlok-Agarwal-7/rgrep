use std::{env, error::Error, fs, process};
use minigrep::{search,search_insensitive};

fn main() {
    let args : Vec<String> = env::args().collect();

    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprint!("problem parsing the args : {err}");
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprint!("there was a problem reading the file :{e}");
        process::exit(1)
    }
}


struct Config{
    query : String,
    filename : String,
    ignore_case : bool
}

impl Config{
   fn build (args : &[String]) -> Result<Config,&str>{
        if args.len()  < 3 {
            return Err("not enough arugments")
        }
        let query = args[1].clone() ;
        let filename =  args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();


        Ok(Config {query,filename,ignore_case})
    }
}

fn run (config : Config) -> Result<(),Box<dyn Error>>{

    let contents = fs::read_to_string(config.filename)?;

    if config.ignore_case {
        for line in search_insensitive( &config.query, &contents){
            println!("{line}")
        }
    } 

    else{
        for line in search( &config.query, &contents){
            println!("{line}")
        }
    }
   
   Ok(())

}

