use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        let line = line.trim();
        // Skip emptpy lines and lines starting with comments
        if line.len() == 0 || line.chars().nth(0) == Some('/') {
            continue;
        }
        if line.chars().nth(0) == Some('@') {
            println!("A Reg");
        }
        else 
        {
            println!("C Reg");
        }
        println!("{}", line);
    }
}

