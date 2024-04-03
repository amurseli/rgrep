use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use rgrep::regex_step::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Uso: {} <regex> <filepath>", args[0]);
        return;
    }

    let regex_str = &args[1];
    let file_path = &args[2];

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return,
    };
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => return,
        };
        lines.push(line);
    }

    for line in lines {
        let pattern = match Regex::new(regex_str) {
            Ok(regex) => regex,
            Err(err) => {
                println!("Error creating regex pattern: {}", err);
                continue;
            }
        };
        for mut regex in pattern {
            match regex.test(&line) {
                Ok(result) => {
                    if !result.is_empty() {
                        println!("{}", result)
                    }
                }
                Err(err) => println!("Error applying the regular expression pattern: {}", err),
            }
        }
    }
}
