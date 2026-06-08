# Caractéristiques et concept du langage

> https://en.wikipedia.org/wiki/Rust_(programming_language)  

Débuté en 2006 et release en 1.0 en Mai 2015, Rust est un langage système qui a comme idée principale d'avoir un langage safe notamment au niveau de la mémoire en proposant un système d'Ownership.

# Forces et limites

- Forces
	- « Si ça compile, ça fonctionne »
		- https://blog.gwadej.org/programmer-musings/2025/05/when-it-compiles-it-works-in-rust/
	- Gestion de mémoire
	- Gestion des threads
	- Outils
	- Bonne documentation
	- Langage compilé
	- Langage majoritairement explicite
	- Typage lourd et statique
		- https://stackoverflow.com/questions/2690544/what-is-the-difference-between-a-strongly-typed-language-and-a-statically-typed
	- Fonctionnalités du langage
		- enum
		- match
		- Option
		- ...
- Limites
	- Frictions avec le borrow checker
	- Temps de compilation
	- Stabilisation de la librairie standard lente
	- Maturité des dépendances pas encore au niveau
		- Pas encore de choix "parfait" pour faire du GUI
		- Possible de faire des jeux mais c'est toujours plus pratique de le faire dans d'autres langages
		- Le web commence a peine a devenir mature

# Resources

- "The Book": https://doc.rust-lang.org/stable/book/
- Rust by example: https://doc.rust-lang.org/rust-by-example
- Documentation librairie standard: https://doc.rust-lang.org/stable/std/index.html ou https://std.rs/
- Cheatsheet: https://cheats.rs/
- Compilation de librairie tierces: https://blessed.rs/crates
- Godbolt (Compiler Explorer): https://godbolt.org/
- Exercices:
	- https://rustlings.rust-lang.org/
	- https://adventofcode.com/
	- https://exercism.org/