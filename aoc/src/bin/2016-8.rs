use std::fs;
use std::io::{self};

enum Action {
    Rectangle(usize, usize),
    Rrow(usize, usize),
    Rcolumn(usize, usize),
}

enum Error {
    SomeKindOfError,
}

fn parse(text: &str) -> Result<Action, Error> {
    let words: Vec<&str> = text.split(' ').collect();

    match words.len() {
        2 => {
            let word = words[1];
            let parts: Vec<usize> = word.split("x").map(|n| n.parse().unwrap_or(0)).collect();
            Ok(Action::Rectangle(parts[0], parts[1]))
        }
        5 => {
            let direction: &str = words[1];
            let shift_value: usize = words[4].parse().unwrap_or(0);
            let index: Vec<&str> = words[2].split("=").collect();

            match direction {
                "row" => Ok(Action::Rrow(index[1].parse().unwrap_or(0), shift_value)),
                "column" => Ok(Action::Rcolumn(index[1].parse().unwrap_or(0), shift_value)),
                _ => Err(Error::SomeKindOfError),
            }
        }
        _ => Err(Error::SomeKindOfError),
    }
}

fn simulate(file_path: &str) -> io::Result<[[bool; 50]; 6]> {
    let mut grid = [[false; 50]; 6];
    let content = fs::read_to_string(file_path)?;
    for line in content.lines() {
        match parse(line) {
            Ok(Action::Rectangle(a, b)) => {
                for row in 0..b {
                    for col in 0..a {
                        grid[row][col] = true;
                    }
                }
            }
            Ok(Action::Rrow(row, by)) => {
                let old = grid[row];
                for col in 0..50 {
                    grid[row][(col + by) % 50] = old[col];
                }
            }
            Ok(Action::Rcolumn(col, by)) => {
                let old: [bool; 6] = std::array::from_fn(|row| grid[row][col]);
                for row in 0..6 {
                    grid[(row + by) % 6][col] = old[row];
                }
            }
            _ => {}
        }
    }
    Ok(grid)
}

fn count_lit_pixel(grid: &[[bool; 50]; 6]) -> usize {
    let count: usize = grid.iter().flatten().filter(|&&p| p).count();
    count
}

fn main() {
    match simulate("input/2016-8.txt") {
        Ok(grid) => {
            let count: usize = count_lit_pixel(&grid);
            println!("{count:?}")
        }
        _ => {}
    }
}
