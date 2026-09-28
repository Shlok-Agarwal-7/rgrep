#[derive(Debug)]
pub struct SearchResult<'a>{
    pub line_number : usize,
    pub line : &'a str 
}

pub fn search<'a>(query : & str ,  contents : & 'a str) -> Vec<SearchResult<'a>>{
    let mut vec = Vec::new();

    for (i ,line) in contents.lines().enumerate(){
        if line.contains(query){
            vec.push(SearchResult { line_number: (i + 1), line });
        }
    }
    vec
}

pub fn search_insensitive<'a>(query : & str ,  contents : & 'a str) -> Vec<SearchResult<'a>>{
    let mut vec = Vec::new();

    let query = query.to_lowercase();

    for (i,line) in contents.lines().enumerate(){
        if line.to_lowercase().contains(&query){
            vec.push(SearchResult { line_number: (i + 1), line });
        }
    }
    vec
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\
Rust:
Simple,fast,productive
Pick Three";

            assert_eq!(vec!["Simple,fast,productive"],search(query,content))
    }

#[test] 
    fn case_insensitive(){
        let query = "siMpLe";
        let content = "\
Rust:
Simple,fast,productive
Pick Three";

            assert_eq!(vec!["Simple,fast,productive"],search_insensitive(query,content))
    }
}