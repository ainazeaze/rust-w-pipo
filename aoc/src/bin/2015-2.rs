use std::fs::File;
use std::io::{self, Read};

fn first_parenthesis_negative_floor(file_path: &str) -> io::Result<i32> {
    let mut count: i32 = 0;
    let mut index: i32 = 0;
    let mut f: File = File::open(file_path)?;
    let mut data: String = String::new();
    f.read_to_string(&mut data)?;

    for c in data.chars() {
        if count == -1 {
            return Ok(index);
        }
        index += 1;
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => {}
        }
    }
    Ok(index)
}

fn main() -> io::Result<()> {
    let count = first_parenthesis_negative_floor("input/2015-1.txt")?;
    println!("{}", count);
    Ok(())
}
