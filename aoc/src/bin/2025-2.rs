use std::fs;

fn is_invalid(n: usize) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even number of digits
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    s[..half] == s[half..]
}

fn main() {
    let content = fs::read_to_string("input/2025-2.txt").unwrap();
    let mut total: u64 = 0;
    for line in content.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        for range in ranges {
            let whatever: Vec<usize> = range.split("-").map(|n| n.parse().unwrap()).collect();
            for id in whatever[0]..=whatever[1] {
                if is_invalid(id) {
                    total += id as u64;
                }
            }
        }
    }
    println!("{total}");
}
