# Librairie standard

> https://doc.rust-lang.org/stable/std/index.html  
> https://std.rs/  

Librairie disponible de base avec le langage. Le [prelude](https://doc.rust-lang.org/stable/std/prelude/index.html) importe automatiquement des elements de la librairie standard (par exemple `String`, `println!`, ...)

# Syntaxe

> https://doc.rust-lang.org/rust-by-example/  
> https://doc.rust-lang.org/stable/std/index.html#keywords  

# Inference

> https://doc.rust-lang.org/rust-by-example/types/inference.html  

# Statements et Expressions

> https://www.compilenrun.com/docs/language/rust/rust-fundamentals/rust-statements-expressions/  
> https://doc.rust-lang.org/rust-by-example/expression.html  

Rust est un langage qui est très orientée expressions avec l'utilisation du paradigme fonctionnel, la présence des blocks et la façon dont sont retournées les variables dans une fonction.

```rust
fn add(a: i32, b: i32) -> i32 {
	// Pas de `return`
	a + b
}


let a = {
	// Pas particulièrement utile mais légal
	50
};

let b = {
	// On peut créer des variables et appeler des fonctions si on veut
	let a = 10;
	let b = 5;
	add(a, b)
};

let c = {
	// On peut même avoir des conditions ou des boucles
	let mut cpt = 0;
	for i in 0..10 {
		cpt += i;
	}
	cpt
};

// Les expressions se trouvent aussi dans le paradigme fonctionnel (ici `map`)
let d: u32 = [1, 2, 3].iter().map(|x| x * 10).sum();
```

# Conditions

> https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html  
> https://doc.rust-lang.org/stable/book/ch19-00-patterns.html  
> https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html  
> https://doc.rust-lang.org/rust-by-example/flow_control/match.html  

Deux moyens de conditionner le code : `if` et `match`.
`if` est classique, comme dans les autres langages, à la différence qu'on ne met pas de parenthèses.
`match` est plus puissant, il permet de conditionner sur les variantes d'une variable. Le `match` doit toujours être exhaustif.

# Boucles

> https://doc.rust-lang.org/rust-by-example/flow_control/for.html  
> https://doc.rust-lang.org/rust-by-example/flow_control/while.html  
> https://doc.rust-lang.org/rust-by-example/flow_control/loop.html  

# Mutabilité

> https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html  
> https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html  

Les variables sont immuables par défaut (ne peut pas être modifié). Il faut rajouter le mot clé `mut` pour les rendre modifiable. Ne pas confondre avec les constantes qui ne va pas utiliser la donnée de la même façon (les constantes n'existent pas au runtime, ils sont remplacé à la compilation).

La mutabilité est associé à la variable (binding) et pas à la valeur. Il s'agit d'une fonctionnalité qui reside uniquement pendant l'étape de compilation :
```
let a = 1;
a = 10; // ERREUR cannot mutate immutable variable

let mut b = a;
b = 20; // OK

let c = b;
c = 30; // ERREUR cannot mutate immutable variable

```

# Types primitifs

> https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#scalar-types  
> https://doc.rust-lang.org/rust-by-example/primitives.html#scalar-types  

Rust ne fournit pas de conversion implicite (coercion) entre les types primitifs :
```
let a: u32 = 5;
let b: u16 = 7;

let c = a + b;        // ERROR
let c = a + b as u32; // OK: type u32
```

# Types composites

> https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#compound-types  
> https://doc.rust-lang.org/rust-by-example/primitives.html#compound-types  

## array

> https://doc.rust-lang.org/stable/std/primitive.array.html  

Tableau de taille fixe d'un type T

```
let mut a: [i32; 3] = [10, 20, 30];

// Ecritures équivalentes
let b: [i32; 3] = [10, 10, 10];
let c: [i32; 3] = [10; 3];

// L'accès d'un element se fait avec la syntaxe "[]"
a[0] = 100;
a[2] = 300;
println!("{} {} {}", a[0], a[1], a[2]); // 100 20 300
a[3] = 400; // ERREUR index out of bounds (compilation)
for i in 0..10 {
	a[i] = 0; // ERREUR index out of bounds (runtime)
}
```

## tuple

> https://doc.rust-lang.org/stable/std/primitive.tuple.html  

Tuple de taille fixe d'un ou plusieurs types.

```
let a: (u32, u32) = (6, 7);
let b: (char, u32) = ('g', 9);

// L'accès d'un element se fait avec la syntaxe "."
a.0 = 60;
println!("{} {}", a.0, a.1); // 60 70
```

# Option et Result

> https://doc.rust-lang.org/stable/std/option/index.html  
> https://doc.rust-lang.org/stable/std/result/index.html  

`Option` est une enum qui permet de contourner l'absence de `null` dans le langage en donnant une variante `None` qui n'a rien et `Some(T)` qui contient une valeur.
`Result` est une enum qui permet de 

# Collections

> https://doc.rust-lang.org/stable/std/collections/index.html  

## Vec

> https://doc.rust-lang.org/stable/std/vec/struct.Vec.html  

```
let mut a: Vec<u32> = Vec::new();
a.push(10);
a.push(20);
a.push(30);

// Équivalent à "a"
let b = vec![10, 20, 30];

// L'accès d'un element se fait avec la syntaxe "[]"
a[0] = 100;
a[2] = 300;
println!("{} {} {}", a[0], a[1], a[2]); // 100 20 300
a[3] = 400; // ERREUR index out of bounds (runtime)
```

## HashMap

> https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html  

## HashSet

> https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html  

# String

> https://cheats.rs/#string-conversions  
> https://fasterthanli.me/articles/working-with-strings-in-rust  

Il existe une multitude de façons de représenter un String en Rust. On peut se focaliser sur deux en particulier :
- `str` : https://doc.rust-lang.org/stable/std/primitive.str.html
- `String` : https://doc.rust-lang.org/stable/std/string/struct.String.html
