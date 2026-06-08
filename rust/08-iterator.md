# References, mutabilité et ownership

> https://doc.rust-lang.org/rust-by-example/flow_control/for.html#for-and-iterators  

Il est possible d'utiliser les iterator en les consommant de différentes façon :
```rust
let vec: Vec<i32> = vec![0, 1, 2];

// value: i32
for value in vec {
	// ...
}

// ERROR: use of moved value
// vec a été consommé, son ownership a été move et n'est plus accessible
for value in vec {
	// ...
}
```

```rust
let vec: Vec<i32> = vec![0, 1, 2];

// value: &i32
for value in &vec {
	// ...
}

// On peut toujours réutiliser vec car on a donné une reference

// value: i32
for value in vec {
	// ...
}
```

```rust
let vec: Vec<i32> = vec![0, 1, 2];

// value: &mut i32
for value in &mut vec {
	// on peut modifier la valeur comme on veut
	*value += 1;
}
```
# Implementer Iterator

> https://doc.rust-lang.org/rust-by-example/trait/iter.html  

```rust
struct MyArray {
    array: [i32; 5],
    index: usize,
}

impl Iterator for MyArray {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let res = self.array[self.index];
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}

fn main() {
    let arr = MyArray {
        array: [1, 2, 3, 4, 5],
        index: 0,
    };

    for i in arr {
        println!("{}", i);
    }
}
```

# IntoIterator

> https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#impl-IntoIterator-for-I  
> https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter  

```rust
struct MyArray {
    array: [i32; 5],
}

struct MyArrayIter {
    array: [i32; 5],
    index: usize,
}

struct MyArrayIterRef<'a> {
    array: &'a [i32; 5],
    index: usize,
}

impl IntoIterator for MyArray {
    type Item = i32;

    type IntoIter = MyArrayIter;

    fn into_iter(self) -> Self::IntoIter {
        MyArrayIter {
            array: self.array,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a MyArray {
    type Item = &'a i32;

    type IntoIter = MyArrayIterRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MyArrayIterRef {
            array: &self.array,
            index: 0,
        }
    }
}

impl Iterator for MyArrayIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let res = self.array[self.index];
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl<'a> Iterator for MyArrayIterRef<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let res = &self.array[self.index];
            self.index += 1;
            Some(res)
        } else {
            None
        }
    }
}

fn main() {
    let arr = MyArray {
        array: [1, 2, 3, 4, 5],
    };

    for i in &arr {
        println!("{}", i);
    }

    for i in arr {
        println!("{}", i);
    }
}
```

# Closures

> https://doc.rust-lang.org/stable/book/ch13-01-closures.html  

Fonctions anonymes qui sont beaucoup utilisés dans le paradigme fonctionnel en Rust.

# Paradigme fonctionnel

La programmation fonctionnelle est un paradigme de programmation ou les programmes sont construits en composant des fonctions et en les appliquant à la chaine. 

Par exemple, en Haskell :
```haskell
add :: Num a => a -> a -> a
add n x = x + n

mult :: Num a => a -> a -> a
mult n x = x * n

divide :: Fractional a => a -> a -> a
divide n x = x / n

chain :: Fractional a => a -> a -> a -> a -> a
chain m x a d = divide d (add a (mult m x))

chain :: Fractional a => a -> a -> a -> a -> a
chain m a d = divide d . add a . mult m

squareEvens :: [Int] -> [Int]
squareEvens xs = map (^2) (filter even xs)
```

Rust intègre principalement ce paradigme par le biais des iterators :
```rust
fn square_evens(xs: &[i32]) -> Vec<i32> {
    xs.iter()
        .filter(|x: &&i32| *x % 2 == 0)
        .map(|x: &i32| x.pow(2))
        .collect()
}
```
