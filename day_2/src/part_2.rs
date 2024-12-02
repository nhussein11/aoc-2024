use utils::file_reader::get_input;

fn is_safe_report(levels: &[i32]) -> bool {
    let increasing = levels.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
    
    let decreasing = levels.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
    
    increasing || decreasing
}

fn is_safe_with_problem_dampener(levels: &[i32]) -> bool {
    if is_safe_report(levels) {
        return true;
    }
    
    for i in 0..levels.len() {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);
        
        if is_safe_report(&modified_levels) {
            return true;
        }
    }
    
    false
}

fn count_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter()
        .filter(|report| is_safe_with_problem_dampener(report))
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
            println!("Number of safe reports with Problem Dampener: {}", safe_reports);
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}