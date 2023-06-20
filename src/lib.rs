use std::fs;
use std::error::Error;
use std::env;


pub fn run(config: Config)->Result<(), Box<dyn Error>>{
    println!("Query: {}", config.query);
    println!("File name: {}", config.filename);
    let contents = fs::read_to_string(config.filename)?;
    let res = if config.case_sensetive {
        search(&config.query, &contents)
    } else {
        search_ignore_case(&config.query, &contents)
    };
    for line in res {
        println!("{}", line);
    }
    return Ok(());
}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensetive: bool
}

impl Config {
    fn new(query: String, filename: String)->Self{
        let case_sensetive = env::var("IGNORE_CASE").is_err();
        return Self { query, filename, case_sensetive };
    }

    pub fn parse_config(args: &Vec<String>)->Result<Self, &str>{
        if args.len() < 3 {
            return Err("Not Enough Arguments ! ! !");
        }
        return Ok(Self::new(args[1].clone(), args[2].clone()));
    }
}


pub fn search<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut res = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            res.push(line);
        }
    }
    return res;
}

pub fn search_ignore_case<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            res.push(line);
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn one_result(){
        let query = "lil";
        let content = "/wee
yeah 
usher 
lIL
and 
lil
john";

        assert_eq!(vec!["lil"], search(query, content));
    }

    #[test]
    fn ignore_case(){
        let query = "3AsbA";
        let contents = "3asba w se9
w stal bze9
zzZ3asba koss dom tsss";

        assert_eq!(vec!["3asba w se9", "zzZ3asba koss dom tsss"], search_ignore_case(query, contents));
    }
}

