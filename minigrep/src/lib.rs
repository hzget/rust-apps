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
//! * It supports both case-sensitive and case-insensitive searching,
//! depending on whether environment variable IGNORE_CASE is set.
//!
//! ## As a library
//!
//! As a library, it provide `Config::build()` and `run()` APIs for
//! the functionality.

#![deny(missing_docs)]

use std::env;
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
    /// an Env variable to turn on/off case sensitive/insensitive searching
    pub ignore_case: bool,
}

const ENV_KEY: &str = "IGNORE_CASE";
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
    /// env::set_var("IGNORE_CASE", "1");
    /// let config = Config::build(cmd).unwrap();
    ///
    /// assert_eq!(
    /// Config {
    ///   query: "searchstring".to_string(),
    ///   file_path: "example.txt".to_string(),
    ///   ignore_case: true,
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

        let ignore_case = env::var(ENV_KEY).is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// takes a configuration and runs the grep functionaly.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;

    let matches = if config.ignore_case {
        search_case_insensitive(&config.query, &text)
    } else {
        search(&config.query, &text)
    };

    for line in matches {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
            ignore_case: false,
        });
        assert!(x.is_err());

        // case 2: case sensitive searching
        let x = run(Config {
            query: "say".to_string(),
            file_path: "poem.txt".to_string(),
            ignore_case: false,
        });
        assert_eq!((), x.expect("REASON"));

        // case 3: case insensitive searching
        let x = run(Config {
            query: "say".to_string(),
            file_path: "poem.txt".to_string(),
            ignore_case: true,
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
        case_config_build_with_env_ignore_case();
        case_config_build_without_env_ignore_case();
    }

    fn case_config_build_with_invalid_args() {
        assert_eq!(
            ERROR_MISS_QUERY,
            Config::build(vec![].into_iter()).unwrap_err()
        );

        let mut v = Vec::new();
        v.push(String::from("minigrep"));
        assert_eq!(
            ERROR_MISS_QUERY,
            Config::build(v.into_iter()).unwrap_err()
        );

        let mut v = Vec::new();
        v.push(String::from("minigrep"));
        v.push(String::from("say"));
        assert_eq!(
            ERROR_MISS_FILEPATH,
            Config::build(v.into_iter()).unwrap_err()
        );
    }

    fn case_config_build_with_env_ignore_case() {
        let cmd = "minigrep searchstring example.txt"
            .split(' ')
            .map(|s| s.to_string());

        let ignore_case = env::var(ENV_KEY).is_ok();
        if !ignore_case {
            env::set_var(ENV_KEY, "1");
        }

        assert_eq!(
            Config {
                query: "searchstring".to_string(),
                file_path: "example.txt".to_string(),
                ignore_case: true,
            },
            Config::build(cmd).unwrap()
        );

        if !ignore_case {
            env::remove_var(ENV_KEY);
        }
    }

    fn case_config_build_without_env_ignore_case() {
        let cmd = "minigrep searchstring example.txt"
            .split(' ')
            .map(|s| s.to_string());

        let ignore_case = env::var(ENV_KEY).is_ok();
        let mut old = String::new();
        if ignore_case {
            old = env::var(ENV_KEY).unwrap();
            env::remove_var(ENV_KEY);
        }

        assert_eq!(
            Config {
                query: "searchstring".to_string(),
                file_path: "example.txt".to_string(),
                ignore_case: false,
            },
            Config::build(cmd).unwrap()
        );

        if ignore_case {
            env::set_var(ENV_KEY, old);
        }
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

    #[test]
    fn case_search_case_insensitive() {
        let s = "hello,
hello me,
say sorry,
say yes,
say Yes or No,
say thank you,";
        assert_eq!(
            vec!["say yes,", "say Yes or No,"],
            search_case_insensitive("Yes", s)
        );
    }
}
