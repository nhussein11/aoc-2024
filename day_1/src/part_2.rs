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

    let mut total_repetitions = Vec::new();

    for number in left_side.iter() {
        let repetitions : i32 = right_side.iter().filter(|&&x| x == *number).count().try_into().unwrap();

        if repetitions > 0 {
            total_repetitions.push(repetitions * number);
        }
    }

    let total_sum: i32 = total_repetitions.iter().sum();
    println!("Total sum of repetitions: {}", total_sum);
}