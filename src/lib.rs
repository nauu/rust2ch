use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub appname: String,
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments ");
        }

        let appname = match args.next(){
            Some(arg) => arg,
            None => return Err("Did not get appname"),
        };

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Did not get query"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Did not get filename"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            appname,
            query,
            filename,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive{
        search(&config.query,&contents)
    }else{
        search_case_insensitive(&config.query,&contents)
    };

    for line in results{
        println!("{}",line);
    }

    //println!("With text:\n{}", contents);
    Ok(())
}

pub fn search<'a>(query: &str,content:&'a str)->Vec<&'a str>{
    // let mut results = Vec::new();

    // for line in content.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // println!("{:?}",results);

    //results

    content.lines().filter(|lines| lines.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str,content:&'a str)->Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    // println!("{:?}",results);
    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."],search(query,content));
    }

    #[test]
    fn case_insensitive(){
        let query = "ruST";
        let content = "Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],search_case_insensitive(query,content));
    }




}