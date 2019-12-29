use std::{env, process, fs, io};
use std::io::prelude::*;
use regex::Regex;


fn open_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Provide path to textfile.");
        process::exit(1);
    }
    
    let file_content = open_file(&args[1]).unwrap();

    println!("{}", file_content);



}