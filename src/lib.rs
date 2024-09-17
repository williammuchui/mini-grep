use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    needle: String,
    haystack: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments;");
        }
        let needle = args[1].clone();
        let haystack = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            needle,
            haystack,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let haystack = fs::read_to_string(config.haystack)?;
    let result = if config.ignore_case {
        case_insensitive_search(&config.needle, &haystack)
    } else {
        case_sensitive_search(&config.needle, &haystack)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn case_sensitive_search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }
    results
}

pub fn case_insensitive_search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let needle = needle.to_lowercase();
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.to_lowercase().contains(&needle) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let needle = "Hello";
        let haystack = "Hello world;\nWelcome to Elclassico";

        assert_eq!(
            case_sensitive_search(needle, haystack),
            vec!["Hello world;"]
        );
    }

    #[test]
    fn two_result() {
        let needle = "Hello";
        let haystack = "hello world from myself again\n and again;";

        assert_eq!(
            case_insensitive_search(needle, haystack),
            vec!["hello world from myself again"]
        );
    }
}
