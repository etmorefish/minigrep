use std::{error::Error, fs, env};


pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config{query, filename}
// }

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            // panic!("not enough arguments")
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, filename, case_sensitive})

    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
        // .expect("Something went wrong reading the file");
    // println!("\nwith text:\n{}", contents);
    
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else{
        search_case_insensitive(&config.query, &contents)
    };
    
    // for line in search(&config.query, &contents) {
    //     println!("{}", line);
    // }
    for line in results{
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    // 使用 lines 方法遍历每一行
    for line in contents.lines() {
        if line.contains(query) {
            // 对文本行进行操作
            // 存储匹配的行
            results.push(line);
        }

    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}