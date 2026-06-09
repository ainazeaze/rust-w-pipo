use std::{fs, io};

fn solve(file_path: &str) -> io::Result<i32> {
    let input = fs::read_to_string(file_path)?;
    let result = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap_or(0))
                .collect();
            is_safe(&numbers)
        })
        .count() as i32;
    Ok(result)
}

fn is_safe(input: &[i32]) -> bool {
    let diffs: Vec<i32> = input.windows(2).map(|w| w[0] - w[1]).collect();
    diffs.iter().all(|&d| d.abs() >= 1 && d.abs() <= 3)
        && (diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0))
}
fn main() {
    let result: i32 = solve("input/2024-2.txt").unwrap();
    println!("{result}")
}
