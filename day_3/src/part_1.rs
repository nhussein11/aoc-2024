use utils::file_reader::get_input;
use regex::Regex;

pub fn main() {
    let input = get_input("day_3/input.txt").unwrap().join("");
    
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total: i32 = re.captures_iter(&input)
        .filter_map(|cap| {
            let x: i32 = cap[1].parse().ok()?;
            let y: i32 = cap[2].parse().ok()?;
            Some(x * y)
        })
        .sum();
    
    println!("Sum of mul() results: {}", total);
}