use rgrep::regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Uso: {} <regex> <filepath>", args[0]);
        return;
    }

    let regex_str = &args[1];
    let file_path = &args[2];

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error al abrir el archivo '{}': {}", file_path, err);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for line in lines {
        let mut pattern = match Regex::new(regex_str) {
            Ok(regex) => regex,
            Err(err) => {
                println!("Error creating regex pattern: {}", err);
                continue;
            }
        };
        for mut regex in pattern{
            
            match regex.test(&line) {
                Ok(result) => {
                    if !result.is_empty(){
                        println!("{}", result)
    
                    }
                },
                Err(err) => println!("Error applying the regular expression pattern: {}", err),
            }
        }
    }
}
