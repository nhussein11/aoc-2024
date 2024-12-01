use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn get_input(path: &str) -> Result<Vec<String>, io::Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    };

    let reader = BufReader::new(file);
    let mut input = Vec::new();

    for line in reader.lines() {
        let line = line?; 
        input.push(line);
    }

    Ok(input)
}