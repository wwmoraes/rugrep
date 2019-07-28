//! # rugrep
//! 
//! `rugrep` provides grep-like utilities with higher performance and lower footprint

use std::error::Error;
use std::fs;

/// Wrapper around [search] fn that defaults the `insensitive` argument to `false`
/// # Examples
/// [search]: fn.search.html
/// ```
/// # #[macro_use] extern crate rugrep; use rugrep::search; fn main() {
/// search!("needle", "hay\nneedle\nhay\nNeedle\n"); // defaults to false
/// search!("needle", "hay\nneedle\nhay\nNeedle\n", false);
/// search!("needle", "hay\nneedle\nhay\nNeedle\n", true);
/// # }
/// ```
#[macro_export]
macro_rules! search {
    ($query:expr, $contents:expr) => {
        search($query, $contents, false)
    };
    ($query:expr, $contents:expr, $insensitive:expr) => {
        search($query, $contents, $insensitive)
    };
}

/// Runs the grep logic with the given `Options` object
/// 
/// # Examples
/// 
/// ```
/// use std::env;
/// use rugrep::{run, Options, OptionsFlags};
/// 
/// // Will print to stderr "Error: not enough arguments"
/// if let Ok(env_options) = Options::new(env::args().skip(1)) {
///     if let Err(message) = run(env_options) {
///         eprintln!("Error: {}", message);
///     };
/// }
/// 
/// let manual_options = Options{
///     needle: String::from("needle"),
///     filenames: vec![String::from("file1.txt")],
///     flags: OptionsFlags{
///         verbose: false,
///         case_insensitive: false,
///     },
/// };
/// if let Err(message) = run(manual_options) {
///     eprintln!("Error: {}", message);
/// };
/// ```
pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    if options.flags.verbose {
        println!("Needle: {:?}", options.needle);
        println!("Files: {:?}", options.filenames);
    }

    for filename in options.filenames.iter() {
        let metadata = fs::metadata(filename)?;

        if !metadata.is_file() {
            eprintln!("{} isn't a file, ignoring\n", filename);
            continue;
        }

        let contents = fs::read_to_string(filename)?;
        for (line_number, line_content) in search!(&options.needle, &contents, options.flags.case_insensitive) {
            println!("{} at {}: {}", filename, line_number, line_content);
        }
    }

    Ok(())
}

/// Search query in content either case-sensitive or case-insensitive
/// 
/// # Examples
/// 
/// ```
/// use rugrep::search;
/// 
/// search("needle", "hay\nneedle\nhay\nNeedle\n", false);
/// ```
pub fn search<'a>(query: &str, content: &'a str, insensitive: bool) -> Vec<(usize, &'a str)> {
    let query: String = match insensitive {
        true => query.to_lowercase(),
        false => query.to_owned(),
    };

    let matcher: Box<Fn(&(usize, &str)) -> bool> = match insensitive {
        true => Box::new(|(_, line): &(usize, &str)| -> bool {
            line.to_lowercase().contains(&query)
        }),
        false => Box::new(|(_, line): &(usize, &str)| -> bool {
            line.contains(&query)
        }),
    };

    content.lines().enumerate().filter(&matcher).map(|(index, line)| (index+1, line)).collect()
}

impl Options {
    /// Parses the `args` into a new `Option` object
    pub fn new<T>(args: T) -> Result<Options, &'static str>
        where T: std::iter::Iterator<Item=String>,
    {
        let mut arg_flags = Vec::new();
        let mut arg_items = Vec::new();

        for arg in args {
            if arg.starts_with("-") {
                arg_flags.push(arg);
            } else {
                arg_items.push(arg);
            }
        }

        if arg_items.len() < 2 {
            return Err("not enough parameters");
        }

        let needle = arg_items.remove(0);
        let filenames = arg_items;

        let flags = OptionsFlags {
            verbose: arg_flags.iter().any(|flag| flag == "-v"),
            case_insensitive: arg_flags.iter().any(|flag| flag == "-i") || std::env::var("CASE_INSENSITIVE").is_ok(),
        };

        Ok(Options {
            flags,
            filenames,
            needle,
        })
    }
}

/// Argument options
pub struct Options {
    pub flags: OptionsFlags,
    pub needle: String,
    pub filenames: Vec<String>,
}

/// Argument flags
pub struct OptionsFlags {
    pub verbose: bool,
    pub case_insensitive: bool,
}

#[cfg(test)]
mod search {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec![(1, "Rust:")],
            search!(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec![(1, "Rust:"), (4, "Trust me.")],
            search!(query, contents, true)
        );
    }
}