use utils::file_reader::get_input;

pub fn main() {
    let input = get_input("day_1/input.txt").unwrap();

    let mut left_side = Vec::with_capacity(input.len());
    let mut right_side = Vec::with_capacity(input.len());

    for line in input {
        let mut parsed_numbers = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .take(2); 

        if let (Some(left), Some(right)) = (parsed_numbers.next(), parsed_numbers.next()) {
            left_side.push(left);
            right_side.push(right);
        } else {
            eprintln!("Error parsing line: {}", line);
        }
    }

    left_side.sort();
    right_side.sort();

    if left_side.len() != right_side.len() {
        eprintln!("Warning: Left and right sides have different lengths!");
    }

    let mut gaps: Vec<i32> = Vec::with_capacity(left_side.len());

    for (left, right) in left_side.iter().zip(right_side.iter()) {
        let gap = (right - left).abs();
        gaps.push(gap);
    }

    let total_sum: i32 = gaps.iter().sum();

    println!("Total sum of gaps: {}", total_sum);
}