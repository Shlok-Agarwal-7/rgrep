
use std::{path::PathBuf};
use clap::{ArgMatches};
pub enum CaseSensitivity{
    Insensitive,
    Sensitive
}

pub struct SearchConfig{
    pub filepaths : Vec<PathBuf>,
    pub query : String,
    pub case_sensitivity : CaseSensitivity,
    pub line_number : bool,
    pub invert_match : bool
}

impl SearchConfig {
    pub fn build(matches : ArgMatches) -> SearchConfig{
        let  query = matches.get_one::<String>("query").expect("query cannot be empty");
        let  filepaths  = matches.get_many::<PathBuf>("filepath").unwrap_or_default().cloned().collect();
        let case_sensitivity = if matches.get_flag("ignore-case") {
                    CaseSensitivity::Insensitive
                } else {
                    CaseSensitivity::Sensitive
                };
        let line_number = matches.get_flag("line-number");
        let invert_match = matches.get_flag("invert_match");
        SearchConfig { filepaths, 
                       query: (query.clone()), 
                       case_sensitivity,
                       line_number,
                       invert_match}
    }
}

pub struct SearchOptions{
    case_sensitivity : CaseSensitivity,
    line_number : bool,
    invert_match : bool
}