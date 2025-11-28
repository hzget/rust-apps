//! # minigrep
//!
//! `minigrep` is both a command line tool and a library to
//! search a string from a text file.
//!
//! ## As a command line tool
//!
//! As a command line tool, it will search a string from
//! a text file and output lines containing that string.
//!
//! Usage:
//!
//! ```bash
//! minigrep searchstring example-filename.txt
//! ```
//!
//! Note:
//!
//! * It writes result to stdout, and writes error info to stderr
//! * It prints helper information if its argment is incorrect
//!
//! ## As a library
//!
//! As a library, it provide `Config::build()` and `run()` APIs for
//! the functionality.

#![deny(missing_docs)]

use std::error::Error;
use std::fs;

/// holds arguments for the grep functionality, as well as
/// options for its behavior.
#[derive(Debug, PartialEq)]
pub struct Config {
    /// the string to search
    pub query: String,
    /// the file to search from
    pub file_path: String,
}

const ERROR_MISS_QUERY: &str = "miss query string in the arguments";
const ERROR_MISS_FILEPATH: &str = "miss filepath in the arguments";

impl Config {
    /// Constructs a config for the grep functionality to run.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::env;
    /// use minigrep::Config;
    ///
    /// let cmd = "minigrep searchstring example.txt".split(' ').map(|s| s.to_string());
    /// let config = Config::build(cmd).unwrap();
    ///
    /// assert_eq!(
    /// Config {
    ///   query: "searchstring".to_string(),
    ///   file_path: "example.txt".to_string(),
    /// },
    /// config);
    ///```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(ERROR_MISS_QUERY),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(ERROR_MISS_FILEPATH),
        };

        Ok(Config { query, file_path })
    }
}

/// takes a configuration and runs the grep functionaly.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;

    let matches = search(&config.query, &text);

    for line in matches {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_run() {
        // case 1: file not exist
        let x = run(Config {
            query: "say".to_string(),
            file_path: "no-exist-file.txt".to_string(),
        });
        assert!(x.is_err());

        // case 2: case sensitive searching
        let x = run(Config {
            query: "say".to_string(),
            file_path: "poem.txt".to_string(),
        });
        assert_eq!((), x.expect("REASON"));
    }

    #[allow(dead_code)]
    fn case_run_output() {
        // TODO
    }

    #[test]
    fn case_build() {
        case_config_build_with_invalid_args();
        case_config_build();
    }

    fn case_config_build_with_invalid_args() {
        assert_eq!(
            ERROR_MISS_QUERY,
            Config::build(vec![].into_iter()).unwrap_err()
        );

        let mut v = Vec::new();
        v.push(String::from("minigrep"));
        assert_eq!(ERROR_MISS_QUERY, Config::build(v.into_iter()).unwrap_err());

        let mut v = Vec::new();
        v.push(String::from("minigrep"));
        v.push(String::from("say"));
        assert_eq!(
            ERROR_MISS_FILEPATH,
            Config::build(v.into_iter()).unwrap_err()
        );
    }

    fn case_config_build() {
        let cmd = "minigrep searchstring example.txt"
            .split(' ')
            .map(|s| s.to_string());

        assert_eq!(
            Config {
                query: "searchstring".to_string(),
                file_path: "example.txt".to_string(),
            },
            Config::build(cmd).unwrap()
        );
    }

    #[test]
    fn case_search() {
        let s = "hello,
hello me,
say sorry,
say Yes,
say yes,
say thank you,";
        assert_eq!(vec!["say yes,"], search("yes", s));
    }
}
