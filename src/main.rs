use std::env::args;
use rgrep::regex::Regex;


fn main() {
    //let args: Vec<&str> = args().collect();

    let pattern = Regex::new("a.b[le]");
    let file_path: &str = "aibe";
    
    //println!("Matching {:?} expression inside {:?}...", pattern, file_path);

    match pattern.unwrap().test(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

}