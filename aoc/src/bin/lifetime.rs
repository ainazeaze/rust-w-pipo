struct StringHandle<'a> {
    s: &'a String,
}

impl<'a> StringHandle<'a> {
    fn new(s: &'a String) -> StringHandle<'a> {
        StringHandle { s }
    }

    fn get_string(&self) -> &'a String {
        self.s
    }
    fn set_string(&mut self, s: &'a String) {
        self.s = s
    }
}

// TODO impl StringHandle

fn main() {
    let a = String::from("foo");
    let b = String::from("bar");

    {
        let mut handle = StringHandle::new(&a);
        println!("{}", handle.get_string()); //foo
        handle.set_string(&b);
        println!("{}", handle.get_string()); //bar
    }

    println!("{} {}", a, b);
}
