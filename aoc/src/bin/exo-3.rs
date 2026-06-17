use std::{array::IntoIter, fmt::Display, ops::Add};

#[derive(Debug)]
struct Pair<T, U>(T, U);

impl<T, U> Pair<T, U> {
    fn sum(self) -> T::Output
    where
        T: Add<U>,
    {
        self.0 + self.1
    }

    fn join<X>(&self, joiner: X) -> String
    where
        T: Display + Clone,
        U: Display + Clone,
        X: Display,
    {
        format!("{}{}{}", self.0.clone(), joiner, self.1.clone())
    }
}

impl<T, U> Default for Pair<T, U>
where
    T: Default,
    U: Default,
{
    fn default() -> Self {
        Pair(T::default(), U::default())
    }
}

impl<T> IntoIterator for Pair<T, T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 2>;
    fn into_iter(self) -> Self::IntoIter {
        [self.0, self.1].into_iter()
    }
}

fn main() {
    let pair = Pair(3, 4);
    println!("{}", pair.sum()); // 7

    let pair = Pair("Hello", "World");
    println!("{}", pair.join(" ")); // Hello World
    println!("{}", pair.join('-')); // Hello-World
    println!("{}", pair.join(0)); // Hello0World

    let pair: Pair<i32, String> = Pair::default();
    println!("{:?}", pair); // Pair(0, "")

    let pair = Pair(6, 7);
    for i in pair {
        println!("{}", i); // 6 then 7
    }
}
