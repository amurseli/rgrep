use std::env::args;



fn main() {
    let args: Vec<String> = args().collect();

    let regex: &String = &args[1];
    let file_path: &String = &args[2];
    
    println!("Matching {:?} expression inside {:?}...",regex,file_path);


}