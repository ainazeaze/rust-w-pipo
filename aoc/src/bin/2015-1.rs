use std::fs::File;
use std::io::{self, Read};

fn count_parenthesis(file_path: &str) -> io::Result<i32> {
    let mut count: i32 = 0;
    let mut f: File = File::open(file_path)?;
    let mut data: String = String::new();
    f.read_to_string(&mut data)?;

    for c in data.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => {}
        }
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    let count = count_parenthesis("input/2015-1.txt")?;
    println!("{:}", count);
    Ok(())
}
