https://adventofcode.com/2016/day/8

# Stack ou Heap ?

```rust
let a = 5;
let b = "Hello";
let mut c = "Hello";
let d = String::from("Hello");
let e = "Hello".to_string();
let f = [10, 20, 30];
let g = vec![10, 20, 30];
let h = Box::new(5);
```

# Traits

```rust
trait Present {
    fn present(&self) -> String;
}

let first_name = FirstName(String::from("James"));
let last_name = LastName(String::from("Bond"));
let full_name = first_name + last_name;
assert_eq!(first_name.present(), String::from("My first name is James"));
assert_eq!(last_name.present(), String::from("Bond is my last name"));
assert_eq!(full_name.present(), String::from("My name is Bond, James Bond"));
```

# Counter Iterator

```rust
// TODO Counter

fn main() {
    let cpt = Counter::new(3, 6);
    for i in cpt {
        println!("{i}");
    }
}

// Output :
// 3
// 4
// 5
// 6
```

# Lifetime

```rust
struct StringHandle<'a> {
    s: &'a String,
}

// TODO impl StringHandle

fn main() {
    let a = String::from("foo");
    let b = String::from("bar");

    {
        let mut handle = StringHandle::new(&a);
        println!("{}", handle.get_string());
        handle.set_string(&b);
        println!("{}", handle.get_string());
    }

    println!("{} {}", a, b);
}
```
