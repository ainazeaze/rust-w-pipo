// Some random note during the course

fn main() {
    println!("Hello, world!");
    let a = 'a';
    println!("{a:#?}"); // debug print
    dbg!(a); // debug print

    // Option null does not exist
    let a: Option<i32> = Some(3);
    let b: Option<i32> = None;
    // cant do a + 2 directly
    let c = a.unwrap() + 2; // but not recommanded, crash in case a is None

    // Exception does not exist, use Result<T,E>, use ? to get the value or return Err/None

    // Collection
    // Vec, HashMap, HashSet
    // Box<dyn, Error>, whatever error
}
