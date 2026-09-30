
#[derive(PartialEq,Debug)]
pub struct SearchResult<'a>{
    pub line_number : usize,
    pub line : &'a str 
}

pub fn search<'a>(query : & str ,  contents : & 'a str,invert_match: bool) -> Vec<SearchResult<'a>>{
    let mut vec_matches  = Vec::new();
    let mut vec_non_matches = Vec::new();

    for (i ,line) in contents.lines().enumerate(){
        if line.contains(query){
            vec_matches.push(SearchResult { line_number: (i + 1), line });
        }
        else{
            vec_non_matches.push(SearchResult{line_number : {i + 1},line});
        }
    }

    if !invert_match{
        return vec_matches;
    }else{
        return vec_non_matches;
    }
}

pub fn search_insensitive<'a>(query : & str ,  contents : & 'a str,invert_match: bool) -> Vec<SearchResult<'a>>{
    let mut vec_matches  = Vec::new();
    let mut vec_non_matches = Vec::new();

    let query = query.to_lowercase();

    for (i,line) in contents.lines().enumerate(){
        if line.to_lowercase().contains(&query){
            vec_matches.push(SearchResult { line_number: (i + 1), line });
        }else{
            vec_non_matches.push(SearchResult{line_number : (i + 1),line});
        }
    }

    if !invert_match{
        return vec_matches;
    }else{
        return vec_non_matches;
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive_matches(){
        let query = "duct";
        let content = "\
Rust:
Simple,fast,productive
Pick Three";
        let result = SearchResult{line_number : 2,line : "Simple,fast,productive"};

            assert_eq!(vec![result],search(query,content,false))
    }

    #[test]
    fn case_sensitive_non_matches(){
        let query = "duct";
        let content = "\
Rust:
Simple,fast,productive
Pick Three";
        let result1 = SearchResult{line_number : 1,line : "Rust:"};
        let result2 = SearchResult{line_number : 3,line : "Pick Three"};

            assert_eq!(vec![result1,result2],search(query,content,true))
    }

    #[test] 
    fn case_insensitive_matches(){
        let query = "siMpLe";
        let content = "\
Rust:
Simple,fast,productive
Pick Three";

        let result = SearchResult{line_number : 2,line : "Simple,fast,productive"};

            assert_eq!(vec![result],search_insensitive(query,content,false))
    }

    #[test]
    fn case_insensitive_non_matches(){
        let query = "SiMplE";
        let content = "\
Rust:
Simple,fast,productive
Pick Three";
        let result1 = SearchResult{line_number : 1,line : "Rust:"};
        let result2 = SearchResult{line_number : 3,line : "Pick Three"};

            assert_eq!(vec![result1,result2],search_insensitive(query,content,true))
    }
}