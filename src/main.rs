use dogrep::config::Config;
use dogrep::search::run;
use std::{env, process};

fn main() {
    // Parse parameter
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    // Run...
    if let Err(err) = run(config) {
        println!("{}", err);
        process::exit(1);
    }
}
