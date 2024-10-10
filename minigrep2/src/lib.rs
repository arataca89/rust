// lib.rs
use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub string: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        args.next();
        
        let string = match args.next() {
            Some(arg) => arg,
            None => return Err("Não foi passada string a ser pesquisada"),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Não foi passado o path do arquivo a ser pesquisado"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{string, filepath, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {

    let file  = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_insensitive(&config.string, &file)
    } else {
        search(&config.string, &file)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "string";
        let contents = "\
linha 1 
linha 2 string
linha 3 ";
        assert_eq!(vec!["linha 2 string"], search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "String";
        let contents = "\
linha 1 
linha 2 String
linha 3 string";
        assert_eq!(vec!["linha 2 String","linha 3 string"], search_insensitive(query,contents));
    }

}