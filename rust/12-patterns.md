> https://rust-unofficial.github.io/patterns/  

# NewType

> https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html  

Permet de renforcer les types utilisés. Par exemple :
```rust
struct Name {
	first_name: String,
	last_name: String,
}

impl Name {
	fn new(first_name: String, last_name: String) -> Self {
		Self { first_name, last_name }
	}
}
```


Permet de contourner la "orphan rule".

Ça peut rendre le code moins pratique si on souhaite utiliser le vrai type dans notre nouveau type, mais on peut implementer Deref pour aider.

# Builder

> https://rust-unofficial.github.io/patterns/patterns/creational/builder.html  