trait Doubler: Sized {
    fn double(&self) -> Self;
}

#[derive(Debug, Clone, Copy, Default)]
struct Number(i32);

impl Doubler for Number {
    //type Output;
    fn double(&self) -> Self {
        Self(0)
    }
}

// Same as derive(clone)
// impl Clone for Number {
//     fn clone(&self) -> Self {
//         Self(self.0)
//     }
// }

// impl Copy for Number {}
// Need trait bound Clone implementation to use Copy

// impl Default for Number {
//     fn default() -> Self {
//         Self(10)
//     }
// }

// impl Drop for Number {
//     fn drop(&mut self) {
//         println!("Je drop sa mere");
//     }
// }
trait Present {
    fn present(&self) -> String;
}

let first_name = FirstName(String::from("James"));
let last_name = LastName(String::from("Bond"));
let full_name = first_name + last_name;
assert_eq!(first_name.present(), String::from("My first name is James"));
assert_eq!(last_name.present(), String::from("Bond is my last name"));
assert_eq!(full_name.present(), String::from("My name is Bond, James Bond"));
fn main() {
    println!("Hello, world!");
    // trait
}
