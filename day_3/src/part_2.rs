use utils::file_reader::get_input;
use regex::Regex;

pub fn main() {
    let input = get_input("day_3/input.txt").unwrap().join("");

    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut is_mul_enabled = true;
    let total: i32 = re_mul.captures_iter(&input)
        .fold(0, |acc, cap| {
            // Check for do() or don't() instructions before mul
            let mul_index = cap.get(0).unwrap().start();
            let do_matches: Vec<_> = re_do.find_iter(&input)
                .filter(|m| m.start() < mul_index)
                .collect();
            let dont_matches: Vec<_> = re_dont.find_iter(&input)
                .filter(|m| m.start() < mul_index)
                .collect();

            is_mul_enabled = if !dont_matches.is_empty() && !do_matches.is_empty() {
                do_matches.last().unwrap().start() > dont_matches.last().unwrap().start()
            } else if !dont_matches.is_empty() {
                false
            } else if !do_matches.is_empty() {
                true
            } else {
                is_mul_enabled
            };

            if is_mul_enabled {
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                acc + x * y
            } else {
                acc
            }
        });

    println!("Sum of enabled mul() results: {}", total);
}