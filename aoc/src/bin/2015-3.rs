use std::fs;
use std::io::{self};

fn resolve(file_path: &str) -> io::Result<u32> {
    let mut count = 0;
    let content = fs::read_to_string(file_path)?;
    for line in content.lines() {
        if line.contains("x") {
            let parts: Vec<u32> = line.split("x").map(|n| n.parse().unwrap_or(0)).collect();
            let [l, w, h] = [parts[0], parts[1], parts[2]];
            let sides = [l * w, w * h, h * l];
            let smallest = sides.iter().min().unwrap_or(&0);
            count += 2 * sides[0] + 2 * sides[1] + 2 * sides[2] + smallest
        }
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    println!("{:?}", resolve("input/2015-3.txt")?);
    Ok(())
}
