use std::time::Instant;
fn main() {
    let n: u64 = 1_000_000_000;
    let mut sum: u64 = 0;

    let start = Instant::now();

    for i in 0..n {
        sum += i;
    }

    let elapsed = start.elapsed();

    println!("Sum:     {}", sum);
    println!("Elapsed: {:.3?}", elapsed);
}
