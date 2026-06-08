> Installation via package manager du système pas recommandé (Ubuntu 24.04 est toujours en 1.74.1)
# Rustup

> https://rustup.rs/  

Outil central pour installer et mettre à jour tout les outils relatif à Rust
# Cargo

> https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html  

Outil central à Rust qui agit comme un package manager aux dépendances d'un projet ou tout simplement un moyen de le manager.
Un simple appel à `cargo` affiche toutes les commandes possibles.
# Clippy

> https://doc.rust-lang.org/stable/book/appendix-04-useful-development-tools.html  
> https://doc.rust-lang.org/stable/clippy/usage.html  

Installé par défaut, permet de checker du code Rust pour tenter de catcher des potentielles erreurs ou l'améliorer.
Se lance avec un appel à la commande `cargo clippy`.
Il peut être configuré dans le `Cargo.toml` dans la section `workspace.lints.clippy`, par exemple :
```toml
[workspace.lints.clippy]
unwrap_used = "warn"
redundant_test_prefix = "warn"
cast_lossless = "warn"
allow_attributes = "warn"
clone_on_ref_ptr = "warn"
create_dir = "warn"
dbg_macro = "warn"
exit = "warn"
implicit_clone = "warn"
unused_result_ok = "warn"
unused_trait_names = "warn"
```

Pour vérifier si clippy est bien configuré, on devrait avoir un warning avec la ligne :
```rust
let _ = 'a'..'z';
```
# Rustfmt

> https://doc.rust-lang.org/stable/book/appendix-04-useful-development-tools.html#automatic-formatting-with-rustfmt  
> https://rust-lang.github.io/rustfmt/  

Installé par défaut, permet de formatter du code Rust.
Se lance avec un appel à la commande `rustfmt` ou `cargo fmt`.
Il peut être configuré par un `rustfmt.toml`, à la racine du projet, par exemple :

```toml
edition = "2024"
max_width = 100
newline_style = "Unix"
```

Pour vérifier si rustfmt est bien configuré, la ligne :
```rust
let _ = 'a'  ..=  'z';
```

devrait être formattée en :
```rust
let _ = 'a'..='z';
```
# Rust-analyzer

> https://doc.rust-lang.org/stable/book/appendix-04-useful-development-tools.html#ide-integration-using-rust-analyzer  
> https://rust-analyzer.github.io/  

Language server (LSP) qui donne accès aux fonctionnalités qu'on peut avoir dans un IDE

rust-analyzer n'est pas installé par défaut

```
rustup component add rust-analyzer
```

ou sur vscode : https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

Configuration d'exemple à définir selon la méthode d'execution (et éditeur utilisé) :
```
check.command = "clippy"
assist.preferSelf = true
inlayHints.bindingModeHints.enable = true
inlayHints.lifetimeElisionHints.enable = "skip_trivial"
```

# Rustlings

> https://rustlings.rust-lang.org/  