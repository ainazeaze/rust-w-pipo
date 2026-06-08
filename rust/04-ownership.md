> https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html  

Concept central de mémoire dans Rust qui est décrit par des règles ce qui garantit la "Memory Safety".

# Variable Scope

> https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variable-scope  
> https://doc.rust-lang.org/rust-by-example/scope/raii.html  

Une variable a un scope ou elle est valide. A la fin de son scope, sa valeur est drop.

```rust
let mut a = Vec::new();

{
    let mut b = Vec::new();
    b.push(10);
} // b est droppé ici

a.extend(b); // ERREUR cannot find value `b` in this scope
```

# Move, Copy et Clone

> https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move  
> https://doc.rust-lang.org/rust-by-example/scope/move.html  

Lorsque la valeur implémente `Copy` et qu'on souhaite l'assigner à une nouvelle variable, la valeur est copiée.
Si la valeur n'implémente pas `Copy`, la valeur est move à ce nouvel owner.
Si la valeur n'implémente pas `Copy` mais qu'elle implémente `Clone`, cela nous donne la possibilité de cloner la valeur (copier manuellement).
On peut savoir si un type implémente `Copy` ou `Clone` dans la documentation de la librairie standard.

```rust
let mut a: i32 = 10; // i32 implemente Copy
let mut b = a; // la valeur 10 est copiée, `a` et `b` sont différents
b = 30;
a = 20; // OK
println!("{} {}", a, b) // 20 30


let mut a = Vec::new(); // Vec n'implemente pas Copy
let mut b = a;
b.push(10);
a.push(20); // ERREUR borrow of moved value: `a`


let mut a = Vec::new(); // Vec n'implemente pas Copy
let mut b = a.clone();
b.push(20);
a.push(10); // OK
println!("{:?} {:?}", a, b); // [10] [20]

fn print_ints(v: Vec<i32>) {
    println!("{:?}", v);
}

let mut a = vec![10, 20, 30];
print_ints(a);
a.push(40);  // ERREUR borrow of moved value: `a`
```

# Borrowing

> https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html  
> https://doc.rust-lang.org/rust-by-example/scope/borrow.html  

Passer une valeur sans changer l'ownership.
Représenté sous forme de reference (comparable aux pointeurs), on peut borrow une reference mutable ou immutable avec les règles suivantes :
- Autant de references immutable tant qu'il n'existe pas de reference mutable en même temps
- Au plus une reference mutable en même temps

```rust
/// Les exemples necessitent que les variables soient bien utilisées pour bien trigger les erreurs

let a = 42;
let a2 = &a; // OK
let a3 = &a; // OK

let mut b = 67;
let b2 = &mut b; // OK

let mut c = 69;
let c2 = &c; // OK
let c3 = &mut c; // ERREUR cannot borrow `c` as mutable because it is also borrowed as immutable

let mut d = 420;
let d2 = &mut d; // OK
let d3 = &mut d; // ERREUR cannot borrow `c` as mutable more than once at a time

let e = 1337;
let e1 = &mut e; // ERREUR cannot mutate immutable variable `c`

fn print_ints(v: &Vec<i32>) {
    println!("{:?}", v);
}

let mut a = vec![10, 20, 30];
print_ints(a);
a.push(40);  // ERREUR borrow of moved value: `a`
```

# Deref

> https://doc.rust-lang.org/stable/std/ops/trait.Deref.html  

Deref est un trait qui permet de définir une façon pour "déreferencer" une valeur pour obtenir une reference à l'interieur de cette dernière.

Un exemple simple est `Vec<T>` qui implemente `Deref` ce qui permet de dereferencer `Vec<T>` pour avoir une reference à l'interieur `&[u8]`.

C'est utile pour avoir des fonctions avec des arguments plus génériques :
```rust
fn print_ints(v: &[i32]) {
    println!("{:?}", v);
}

let a = [10, 20, 30];
print_ints(&a); // OK

let b = vec![10, 20, 30];
print_ints(&b); // OK
```
