use utils::file_reader::get_input;

fn is_safe_report(levels: &[i32]) -> bool {
    let increasing = levels.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
    
    let decreasing = levels.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
    
    increasing || decreasing
}

fn count_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter()
        .filter(|report| is_safe_report(report))
        .count()
}

pub fn main() {
    let input_path = "day_2/input.txt";
    
    match get_input(input_path) {
        Ok(lines) => {
            let reports: Vec<Vec<i32>> = lines
                .iter()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect();
            
            let safe_reports = count_safe_reports(&reports);
            println!("Number of safe reports: {}", safe_reports);
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}