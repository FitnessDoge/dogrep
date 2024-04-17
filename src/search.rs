use crate::config::Config;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read contents from file
    let content = fs::read_to_string(&config.filename)?;

    if config.count {
        // Count mode
        match count(config, &content) {
            0 => println!("There are no lines in the file that match the pattern."),
            1 => println!("There is 1 line in the file that match the pattern."),
            total => println!("There are {total} lines in the file that match the pattern."),
        }
    } else {
        // Search mode
        let res = search(config, &content);
        for line in &res {
            println!("{line}");
        }
    }
    Ok(())
}

/**
 * Search content that match the pattern
 * Return the line number and line content
 */
fn matcher<'a>(config: &'a Config, content: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    content.lines().enumerate().filter(|(_, line)| {
        let flag = if config.ignore_case {
            // Ignore case
            let pattern = config.pattern.to_lowercase();
            line.to_lowercase().contains(&pattern)
        } else {
            // Case insensitive
            line.contains(&config.pattern)
        };

        flag ^ config.reversed
        // if config.reversed {
        //     !flag
        // } else {
        //     flag
        // }
    })
}

/// Calculate the number of lines that match the pattern
fn count(config: Config, content: &str) -> usize {
    matcher(&config, content).count()
}

fn search(config: Config, content: &str) -> Vec<String> {
    if config.line_numbers {
        matcher(&config, content)
            .map(|(index, line)| (index + 1).to_string() + ": " + line)
            .collect()
    } else {
        matcher(&config, content)
            .map(|(_, line)| String::from(line))
            .collect()
    }
}
