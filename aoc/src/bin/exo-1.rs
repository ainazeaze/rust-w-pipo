use std::ops::Add;

trait Present {
    fn present(&self) -> String;
}

#[derive(Clone)]
struct FirstName(String);

#[derive(Clone)]
struct LastName(String);

#[derive(Clone)]
struct FullName(String, String);

impl Present for FirstName {
    fn present(&self) -> String {
        format!("My first name is {}", &self.0)
    }
}

impl Present for LastName {
    fn present(&self) -> String {
        format!("{} is my last name", &self.0)
    }
}

impl Present for FullName {
    fn present(&self) -> String {
        format!("My name is {}, {} {}", &self.1, &self.0, &self.1)
    }
}

impl Add<LastName> for FirstName {
    type Output = FullName;

    fn add(self, other: LastName) -> Self::Output {
        FullName(self.0, other.0)
    }
}

fn main() {
    let first_name = FirstName(String::from("James"));
    let last_name = LastName(String::from("Bond"));

    let fn_clone = first_name.clone();
    let ln_clone = last_name.clone();
    let full_name = first_name + last_name;

    assert_eq!(fn_clone.present(), String::from("My first name is James"));
    assert_eq!(ln_clone.present(), String::from("Bond is my last name"));
    assert_eq!(
        full_name.present(),
        String::from("My name is Bond, James Bond")
    );
}
