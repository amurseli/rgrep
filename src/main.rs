use std::env::args;
use rgrep::regex::Regex;


fn main() {
    //let args: Vec<&str> = args().collect();

    let pattern = Regex::new("ab.*c");
    let file_path: &str = "abec";
    
    //println!("Matching {:?} expression inside {:?}...", pattern, file_path);

    match pattern.unwrap().test(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

}