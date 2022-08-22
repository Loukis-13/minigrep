use std::{error::Error, fs};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "Loukis <github.com/Loukis-13>", 
    version, 
    about = "Simple Rust implementation of the Unix tool `grep`", 
    long_about = None
)]
pub struct Config {
    #[clap(forbid_empty_values = true, help = "Text to search")]
    pub query: String,

    #[clap(parse(from_os_str), help = "File to search in")]
    pub file_path: std::path::PathBuf,

    #[clap(short = 'i', long, value_parser, default_value_t = false, help = "")]
    pub case_insensitive: bool,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        return Ok(Config::parse());
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, file_path: &'a str) -> Vec<&'a str> {
    file_path
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, file_path: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    file_path
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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

