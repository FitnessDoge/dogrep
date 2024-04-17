use std::collections::HashSet;
use std::env::Args;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub pattern: String,
    pub ignore_case: bool,
    pub reversed: bool,
    pub line_numbers: bool,
    pub count: bool,
}

/// Implement Display Trait for Config
impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Config {
    /**
     * Crate a Config object with parameters.
     */
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        // At least three parameters
        if args.len() < 3 {
            return Err("Not enough parameters.");
        }

        // Skip the first parameter(program name)
        args.next();
        let mut args = args.rev();

        // Filename and pattern
        let filename = args
            .next()
            .ok_or("Unexpected error occurred, please try again.")?;
        let pattern = args
            .next()
            .ok_or("Unexpected error occurred, please try again.")?;

        // Other options
        let options: HashSet<_> = args.collect();

        // Build Config object
        Ok(Config {
            pattern,
            filename,
            ignore_case: options.contains("-i"),
            reversed: options.contains("-v"),
            line_numbers: options.contains("-n"),
            count: options.contains("-c"),
        })
    }
}
