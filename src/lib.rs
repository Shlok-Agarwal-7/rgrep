pub fn search<'a>(query : & str ,  contents : & 'a str) -> Vec<&'a str>{
    let mut vec = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            vec.push(line);
        }
    }

    vec
}

pub fn search_insensitive<'a>(query : & str ,  contents : & 'a str) -> Vec<&'a str>{
    let mut vec = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            vec.push(line);
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