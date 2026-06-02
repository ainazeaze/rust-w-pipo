use std::io::{self};

use std::fs;
enum Direction {
    Right(u32),
    Left(u32),
}

enum Error {
    SomeKindOfError,
}

fn parse(text: &str) -> Result<Direction, Error> {
    match text.chars().nth(0) {
        Some('R') => {
            let digit: u32 = text[1..].parse().unwrap_or(0) as u32;
            Ok(Direction::Right(digit))
        }
        Some('L') => {
            let digit: u32 = text[1..].parse().unwrap_or(0) as u32;
            Ok(Direction::Left(digit))
        }
        _ => Err(Error::SomeKindOfError),
    }
}

fn solve(file_path: &str) -> io::Result<i32> {
    let mut dial: i32 = 50;
    let mut count_zero: i32 = 0;
    let content = fs::read_to_string(file_path)?;
    for line in content.lines() {
        match parse(line) {
            Ok(Direction::Right(value)) => {
                for _ in 0..value {
                    dial = (dial + 1) % 100;
                    if dial == 0 {
                        count_zero += 1;
                    }
                }
            }
            Ok(Direction::Left(value)) => {
                for _ in 0..value {
                    dial = (dial - 1 + 100) % 100;
                    if dial == 0 {
                        count_zero += 1;
                    }
                }
            }
            Err(_) => {}
        }
    }
    Ok(count_zero)
}
fn main() {
    let res: i32 = solve("input/2025-1.txt").unwrap();
    println!("{res:?}")
}
