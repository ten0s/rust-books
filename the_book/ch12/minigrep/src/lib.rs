use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
        // skip program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case(&config.query, &contents)
    } else {
        search_icase(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_icase<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    /*
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
     */
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case(query, contents)
        );
    }

    #[test]
    fn search_icase_test() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_icase(query, contents));
    }
}
